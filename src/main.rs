pub mod core;
pub mod special;
pub mod value;
pub mod environment;
pub mod macros;
pub mod evaluator;
pub mod parser;
use anyhow::Result;
use evaluator::*;
use crate::{core::core, value::Value};
use edn_rs::{self, Edn};
use std::str::FromStr;

// crude repl for testing
// input must be one line
async fn repl() -> Result<()> {
    let pipe = std::io::stdin();

    let mut eval = Evaluator::from_core(core());

    let repl = pipe
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| Edn::from_str(&line))
        .filter_map(|x| x.ok())
        .map(|edn| eval.eval(&edn))
        .filter_map(|res| res.ok())
        .map(|res| {println!("{:?}", &res); res})
        .collect::<Vec<Value>>();
    println!("{:?}", repl);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    repl().await?;
    Ok(())
}
