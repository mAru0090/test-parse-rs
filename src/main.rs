mod parser;
mod traits;
mod types;
use crate::parser::syntax::Parser;
use anyhow::Result as R;
use log::*;
use logos::Logos;
use simple_logger::SimpleLogger;

fn main() -> R<()> {
    SimpleLogger::new().init()?;
    let input = r#" let a = """#;
    let mut parser = Parser::new(input);

    if let Some(ast) = parser.expr() {
        debug!("AST: {:?}", ast);
        debug!("Result: {:?}", ast.eval());
    } else {
        error!("Failed to parse expression");
    }

    Ok(())
}
