use crate::traits::*;
use crate::types::*;
use anyhow::Result as R;
use backtrace::Backtrace;
use log::*;
use logos::Logos;
use rustc_demangle::demangle;
use simple_logger::SimpleLogger;
use std::panic::Location;

fn get_function_name(backtrace: &Backtrace) -> Option<String> {
    for frame in backtrace.frames() {
        for symbol in frame.symbols() {
            if let Some(name) = symbol.name() {
                //debug!("Symbol: {:?}", name);  // ここで全シンボル名を表示
            }
        }
    }

    // 通常の処理
    backtrace.frames().get(1).map(|frame| {
        frame
            .symbols()
            .get(0)
            .and_then(|symbol| {
                symbol.name().map(|name| {
                    let demangled_name = demangle(name.as_str().expect("")).to_string();
                    let function_name = demangled_name.split('<').next().unwrap_or(&demangled_name);
                    if function_name.contains("::") {
                        function_name.split("::").last().unwrap_or(function_name)
                    } else {
                        function_name
                    }
                    .to_string()
                })
            })
            .unwrap_or_else(|| "unknown".to_string())
    })
}

pub struct Parser<'a> {
    lexer: logos::Lexer<'a, Token>,
    current: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Token::lexer(input);
        let current = lexer.next().and_then(|res| res.ok());
        Self { lexer, current }
    }

    pub fn advance(&mut self) {
        self.current = self.lexer.next().and_then(|res| res.ok());
    }

    pub fn parse_value_assignment(&mut self) -> Option<Node> {
        None
    }
    pub fn parse_value_definition(&mut self) -> Option<Node> {
        if let Some(token) = &self.current {
            return match token {
                Token::Ident => {
                    //debug!("keyword: {:?}", self.lexer.slice());
                    self.advance(); // let keyword skip

                    //debug!("var_name: {:?}", self.lexer.slice());
                    let ident = self.lexer.slice();
                    self.advance(); // ident skip

                    let data_type = if self.lexer.slice() == ":" {
                        self.advance(); // colon skip
                        Some(self.lexer.slice())
                    } else {
                        None
                    };

                    //debug!("data_type: {:?}", data_type);
                    if data_type.is_some() {
                        self.advance(); // = skip
                        self.advance(); // = skip
                    } else {
                        self.advance(); // = skip
                    }

                    //panic!("{:?}",self.lexer.slice());
                    let data_value = self.expr()?;
                    //debug!("data_value: {:?}", data_value);

                    let value = Some(Node::VariableDefinition(
                        ident.to_string(),
                        data_type.map_or_else(|| None, |v| Some(v.to_string())),
                        Box::new(data_value),
                    ));
                    debug!("Parsed Value Definition to -> {:?}", value);
                    value
                }
                _ => None,
            };
        } else {
            None
        }
    }
    pub fn is_assignment_target_identifier(&mut self) -> bool {
        false
    }

    pub fn is_assignment_target_literal(&mut self) -> bool {
        false
    }

    pub fn factor(&mut self) -> Option<Node> {
        if let Some(token) = &self.current {
            debug!(
                "Current factor token type: {:?} value: {:?}",
                token,
                self.lexer.slice()
            );
            match token {
                Token::Int10Literal => {
                    let value = self.lexer.slice().parse::<i32>().ok()?;
                    self.advance();
                    Some(Node::IntLiteral(value))
                }
                Token::Int16Literal => {
                    let value = i32::from_str_radix(&self.lexer.slice()[2..], 16).ok()?;
                    self.advance();
                    Some(Node::IntLiteral(value))
                }
                Token::Int2Literal => {
                    let value = i32::from_str_radix(&self.lexer.slice()[2..], 2).ok()?;
                    self.advance();
                    Some(Node::IntLiteral(value))
                }
                Token::Int8Literal => {
                    let value = i32::from_str_radix(&self.lexer.slice()[2..], 8).ok()?;
                    self.advance();
                    Some(Node::IntLiteral(value))
                }
                Token::StringLiteral => {
                    let value = self.lexer.slice();
                    self.advance();
                    Some(Node::StringLiteral(value.to_string()))
                }
                Token::CharLiteral => {
                    let _value = self.lexer.slice();
                    self.advance();
                    let value = if let Some(c) = _value.chars().next() {
                        Some(c)
                    } else {
                        None
                    };
                    Some(Node::CharLiteral(value?))
                }
                Token::LParent => {
                    self.advance(); // '(' をスキップ
                    let value = self.expr(); // 括弧内の式を再帰的に評価
                    if self.current == Some(Token::RParent) {
                        self.advance(); // ')' をスキップ
                        value
                    } else {
                        None // エラー: 対応する ')' がない
                    }
                }
                Token::Ident => {
                    let ident = self.lexer.slice();
                    let value = Some(Node::Ident(ident.to_string()));
                    // キーワード処理
                    if Parser::is_keyword_lists(ident) {
                        match ident.as_ref() {
                            "let" => {
                                let value = self.parse_value_definition();
                                return value;
                            }
                            _ => {}
                        }
                    }
                    // 変数定義,代入処理
                    //

                    self.advance(); // ident をスキップ
                    value
                }
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn term(&mut self) -> Option<Node> {
        let mut value = self.factor(); // Remove the `?` operator here
        while let Some(token) = &self.current {
            debug!(
                "Current term token type: {:?} value: {:?}",
                token,
                self.lexer.slice()
            );
            match token {
                Token::Mul => {
                    self.advance();
                    value = Some(Node::Mul(Box::new(value?), Box::new(self.factor()?))); // Fix `Option<Node>` mismatch
                }
                Token::Div => {
                    self.advance();
                    value = Some(Node::Div(Box::new(value?), Box::new(self.factor()?))); // Fix `Option<Node>` mismatch
                }
                _ => break,
            }
        }
        value // Ensure this returns `Option<Node>`
    }

    pub fn expr(&mut self) -> Option<Node> {
        let mut value = self.term(); // Remove the `?` operator here
        while let Some(token) = &self.current {
            debug!(
                "Current expr token type: {:?} value: {:?}",
                token,
                self.lexer.slice()
            );
            match token {
                Token::Add => {
                    self.advance();
                    value = Some(Node::Add(Box::new(value?), Box::new(self.term()?))); // Fix `Option<Node>` mismatch
                }
                Token::Sub => {
                    self.advance();
                    value = Some(Node::Sub(Box::new(value?), Box::new(self.term()?))); // Fix `Option<Node>` mismatch
                }
                _ => break,
            }
        }
        value // Ensure this returns `Option<Node>`
    }

    pub fn is_keyword_lists(s: &str) -> bool {
        let list = vec!["let", "if", "for", "while", "match"];
        list.contains(&s)
    }
}

impl Node {
    pub fn eval(&self) -> Option<DataValue> {
        match self {
            Node::IntLiteral(value) => Some(DataValue::Int32(*value)),
            Node::StringLiteral(value) => Some(DataValue::String(value.clone())),
            Node::CharLiteral(value) => Some(DataValue::Char(*value)),
            Node::Add(left, right) => {
                let left_value = left.eval()?;
                let right_value = right.eval()?;

                match (left_value, right_value) {
                    (DataValue::Int32(left), DataValue::Int32(right)) => {
                        Some(DataValue::Int32(left + right))
                    }
                    _ => None, // 他の型の加算はサポートしていない場合
                }
            }

            Node::Sub(left, right) => {
                let left_value = left.eval()?;
                let right_value = right.eval()?;

                match (left_value, right_value) {
                    (DataValue::Int32(left), DataValue::Int32(right)) => {
                        Some(DataValue::Int32(left - right))
                    }
                    _ => None, // 他の型の引き算はサポートしていない場合
                }
            }

            Node::Mul(left, right) => {
                let left_value = left.eval()?;
                let right_value = right.eval()?;

                match (left_value, right_value) {
                    (DataValue::Int32(left), DataValue::Int32(right)) => {
                        Some(DataValue::Int32(left * right))
                    }
                    _ => None, // 他の型の掛け算はサポートしていない場合
                }
            }

            Node::Div(left, right) => {
                let left_value = left.eval()?;
                let right_value = right.eval()?;

                match (left_value, right_value) {
                    (DataValue::Int32(left), DataValue::Int32(right)) => {
                        if right == 0 {
                            None // 0で割る場合はNoneを返す
                        } else {
                            Some(DataValue::Int32(left / right))
                        }
                    }
                    _ => None, // 他の型の割り算はサポートしていない場合
                }
            }
            //Node::Ident(ident) => Some(DataValue::String(ident.to_string())),
            Node::VariableDefinition(var_name, data_type, value) => {
                debug!(
                    "Define statement var_name: {:?} data_type: {:?} value: {:?}",
                    var_name, data_type, value
                );
                Some(DataValue::Null)
            }
            _ => None,
        }
    }
}
