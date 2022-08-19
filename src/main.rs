use anyhow::Result;

mod ir;
mod parser;

fn main() -> Result<()> {
    parser::parse()?;
    Ok(())
}
