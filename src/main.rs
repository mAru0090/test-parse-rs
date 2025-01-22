

mod parser;
mod types;

use anyhow::Result as R;
use log::*;
use logos::Logos;
use simple_logger::SimpleLogger;
use crate::parser::syntax::Parser;


fn main() -> R<()> {
    SimpleLogger::new().init()?;
    let input = "(3 + 5) * (2 + 4)";
    let mut parser = Parser::new(input);

    if let Some(ast) = parser.expr() {
        println!("AST: {:?}", ast);
        println!("Result: {}", ast.eval());
    } else {
        println!("Failed to parse expression");
    }

    Ok(())
}
