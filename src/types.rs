use anyhow::Result as R;
use log::*;
use logos::Logos;
use simple_logger::SimpleLogger;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[token("(")]
    LParent,
    #[token(")")]
    RParent,
    #[token("=")]
    Equal,
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token(":")]
    Colon,
    #[regex(r#""[^"]*""#)]
    StringLiteral,
    #[regex(r#"'[^']*'"#)]
    CharLiteral,
    #[regex(r"[0-9]+", priority = 1)] // Int literals have higher priority
    Int10Literal,
    #[regex(r"0b[01]+")]
    Int2Literal,
    #[regex(r"0o[0-7]+")]
    Int8Literal,
    #[regex(r"0x[0-9a-fA-F]+")]
    Int16Literal,
    #[regex(r"([a-zA-Z_][a-zA-Z0-9_]*)")]
    Ident,
}

#[derive(Debug)]
pub enum Node {
    IntLiteral(i32),
    StringLiteral(String),
    CharLiteral(char),
    Ident(String),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    // 新しく追加したVariableDefinition
    VariableDefinition(
        String,         // 変数名
        Option<String>, // 変数の型 (例: "int", "char"など)
        Box<Node>,      // 代入する値
    ),
    // 代入のノード
    Assignment(
        String,    // 変数名
        Box<Node>, // 代入する値
    ),
}

#[derive(Debug)]
pub enum DataValue {
    Int64(i64),
    Int32(i32),
    Int16(i16),
    Int8(i8),
    String(String),
    Char(char),
    Null,
}
