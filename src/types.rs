use anyhow::Result as R;
use log::*;
use logos::Logos;
use simple_logger::SimpleLogger;


#[derive(Logos, Debug, PartialEq)]
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
    #[regex(r#""[^"]*""#)]
    StringLiteral,
    #[regex(r#"'[^']*'"#)]
    CharLiteral,
    #[regex(r"[0-9]+")]
    Int10Literal,
    #[regex(r"0b[01]+")]
    Int2Literal,
    #[regex(r"0o[0-7]+")]
    Int8Literal,
    #[regex(r"0x[0-9a-fA-F]+")]
    Int16Literal,
    #[regex("[a-zA-Z]+")]
    Ident,
}

