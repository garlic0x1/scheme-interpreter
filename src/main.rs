pub mod core;
pub mod evaluator;
pub mod expression;
pub mod parser;
pub mod tokenizer;
use anyhow::Result;
use evaluator::*;
use parser::print_val;
use crate::core::core;
use edn_rs::{self, Edn};
use std::{str::FromStr, io::BufRead};

fn repl() -> Result<()> {
    let pipe = std::io::stdin();
    pipe.lines()
        .filter_map(|x| x.ok())
        .map(|line| Edn::from_str(&line))
        .filter_map(|x| x.ok())
        .map(|edn| evaluate(&edn, &mut vec!(core())))
        .filter_map(|res| res.ok())
        .map(|res| print_val(&res))
        .collect::<Vec<Result<Value>>>();
    Ok(())
}
fn test() -> Result<()> {
    let lisp = "((lambda [fn test] (fn test)) (lambda [input] (display input)) {:key \"val\" :key2 4.206e9})";

    let expr = Edn::from_str(lisp)?;

    let res = evaluate(&expr, &mut vec!(core()))?;
    let _: Option<bool> = match res {
        Value::Proc(_) => {dbg!("proc"); None},
        Value::Lambda(l) => {dbg!(l); None},
        Value::Expr(e) => {dbg!(e); None}
    };

    if let Edn::List(list) = expr {
        let mut vect = list.to_vec();
        vect.remove(0);
    } else {
        println!("not list");
    }


    Ok(())
}

fn main() -> Result<()> {
    test()?;
    repl()?;
    Ok(())
}
