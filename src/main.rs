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
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Arguments {
    /// Run a file
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// evaluate a string
    Eval { lisp: String },
    /// run a file as a program
    Run { file: String },
    /// start a REPL
    Repl,
}

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

fn eval(lisp: &str) -> Result<()> {
    let mut eval = Evaluator::from_core(core());
    let res = eval.eval(&Edn::from_str(lisp)?)?;
    println!("{}", res.to_string());
    Ok(())
}

fn run(filename: &str) -> Result<()> {
    let mut eval = Evaluator::from_core(core());
    let lisp = format!("(load \"{}\")", filename);
    eval.eval(&Edn::from_str(&lisp)?)?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Arguments::parse();

    match &args.command {
        Commands::Eval { lisp } => eval(lisp),
        Commands::Run { file } => run(file),
        Commands::Repl => repl(),
    }
}
