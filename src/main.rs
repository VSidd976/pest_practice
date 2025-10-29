use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result<(), pest::error::Error<Rule>> 
{
    let a = Grammar::parse(Rule::file, "-123.456,-151234475647,-156748\n")?;
    println!("{:?}", a);

    Ok(())
}
