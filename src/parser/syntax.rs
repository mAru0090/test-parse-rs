use anyhow::Result as R;
use log::*;
use logos::Logos;
use simple_logger::SimpleLogger;
use crate::types::*;


#[derive(Debug)]
pub enum Node {
    IntLiteral(i32),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
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

    pub fn factor(&mut self) -> Option<Node> {
        if let Some(token) = &self.current {
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
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn term(&mut self) -> Option<Node> {
        let mut value = self.factor(); // Remove the `?` operator here
        while let Some(token) = &self.current {
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
}

impl Node {
    pub fn eval(&self) -> i32 {
        match self {
            Node::IntLiteral(value) => *value,
            Node::Add(left, right) => left.eval() + right.eval(),
            Node::Sub(left, right) => left.eval() - right.eval(),
            Node::Mul(left, right) => left.eval() * right.eval(),
            Node::Div(left, right) => left.eval() / right.eval(),
        }
    }
}

