use anyhow::Result;
use edn_rs::{Edn, EdnError};
use std::str::FromStr;

use crate::evaluator::Value;

pub fn read(input: &str) -> Result<Edn, EdnError> {
    Edn::from_str(input)
}

pub fn print(edn: &Edn) -> Result<String> {
    let output = edn.to_string();
    dbg!(&output);
    Ok(output.to_string())
}

pub fn print_val(input: &Value) -> Result<Value> {
        match input {
            Value::Proc(_) => {
                print!("proc");
            },
            Value::Lambda(l) => {
                print!("{:?}", l);
            },
            Value::Expr(e) => {
                println!("{}", e.to_string());
            },
        }
        Ok(Value::Expr(Edn::Nil))
}
