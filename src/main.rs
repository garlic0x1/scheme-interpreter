pub mod core;
pub mod special;
pub mod value;
pub mod environment;
pub mod macros;
pub mod evaluator;
pub mod parser;
use anyhow::Result;
use evaluator::*;
use crate::core::core;
use edn_rs::{self, Edn};
use std::{str::FromStr, io::Write};

// crude repl for testing
// input must be one line
fn repl() -> Result<()> {
    let pipe = std::io::stdin();
    let mut eval = Evaluator::from_core(core());

    loop {
        print!("{}", "lisp> ");
        std::io::stdout().lock().flush()?;
        let mut line = String::new();
        pipe.read_line(&mut line)?;

        if line == ",quit".to_string() {
            break;
        }

        let edn = Edn::from_str(&line)?;
        println!(";; => {}", eval.eval(&edn)?);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    repl()?;
    Ok(())
}
