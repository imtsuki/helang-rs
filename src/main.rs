use anyhow::Result;
use clap::Parser;
use std::io::Write;

mod interpreter;
mod ir;
mod parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    source_file: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut interpreter = interpreter::Interpreter::new();
    if let Some(source_file) = args.source_file {
        let source = std::fs::read_to_string(source_file)?;
        let stmts = parser::parse(&source)?;
        interpreter.eval(stmts)?;
    } else {
        loop {
            let mut source = String::new();
            write!(std::io::stdout(), "helang> ")?;
            std::io::stdout().flush()?;
            std::io::stdin().read_line(&mut source)?;
            match respond(&source, &mut interpreter) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            }
        }
    }
    Ok(())
}

fn respond(source: &str, interpreter: &mut interpreter::Interpreter) -> Result<()> {
    let stmts = parser::parse(source)?;
    interpreter.eval(stmts)?;
    Ok(())
}
