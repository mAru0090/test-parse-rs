mod parser;
mod traits;
mod types;
use crate::parser::syntax::Parser;
use anyhow::Result as R;
use log::*;
use logos::Logos;
use simple_logger::SimpleLogger;

use clap::{Arg, Parser as ClapParser};

#[derive(Debug, ClapParser)]
struct Args {
    /// Dev test flag (optional)
    #[arg(long)]
    dev_test: bool,
    /// Execute argument
    #[arg(short, long, required = false)]
    execute: Option<String>, // `Option<String>`にして、ない場合も対応
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // dev_testがtrueならexecuteはオプションとして扱う
    let input = if args.dev_test {
        r#"let a = '1'"#
    } else {
        if let Some(execute) = args.execute {
            &execute.clone()
        } else {
            return Err(anyhow::anyhow!(
                "The 'execute' argument is required when 'dev_test' is not specified"
            )
            .into());
        }
    };

    simple_logger::SimpleLogger::new().init()?;
    let mut parser = parser::syntax::Parser::new(input);

    if let Some(ast) = parser.expr() {
        log::debug!("AST: {:?}", ast);
        log::debug!("Result: {:?}", ast.eval());
    } else {
        log::error!("Failed to parse expression");
    }

    Ok(())
}
