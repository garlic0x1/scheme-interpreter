pub mod core;
pub mod value;
pub mod environment;
pub mod macros;
pub mod evaluator;
pub mod parser;
use anyhow::Result;
use environment::*;
use evaluator::*;
use crate::{core::core, value::Value};
use edn_rs::{self, Edn};
use std::str::FromStr;

fn repl() -> Result<()> {
    let pipe = std::io::stdin();
    let mut env = Environment::from_frame(core());
    let repl = pipe
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| Edn::from_str(&line))
        .filter_map(|x| x.ok())
        .map(|edn| evaluate(&edn, &mut env))
        .filter_map(|res| res.ok())
        .map(|res| {println!("{:?}", &res); res})
        .collect::<Vec<Value>>();
    println!("{:?}", repl);
    Ok(())
}

fn main() -> Result<()> {
    repl()?;
    Ok(())
}
