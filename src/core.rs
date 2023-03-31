use crate::{value::*, evaluator::Evaluator};
use anyhow::{anyhow, Result, bail};
use edn_rs::Edn;
use maplit::hashmap;
use std::{collections::HashMap, str::FromStr};

#[macro_export]
macro_rules! str {
    () => {
        String::new()
    };
    ($x:expr $(,)?) => {
        ToString::to_string(&$x)
    };
}

fn slurp(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    if let Some(Value::Expr(Edn::Str(filename))) = input.first() {
        let text = std::fs::read_to_string(filename)?;
        Ok(Value::Expr(Edn::Str(text)))
    } else {
        Err(anyhow!("Bad input {:?}", input))
    }
}

fn cons(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    if let (Some(car), Some(cdr)) = (input.get(0), input.get(1)) {
        if let (Value::Expr(car), Value::Expr(cdr)) = (car, cdr) {
            if let Edn::List(list) = cdr {
                let mut v = list.clone().to_vec();
                v.insert(0, car.clone());
                let res = edn_rs::List::new(v);
                return Ok(Value::Expr(Edn::List(res)));
            }
        }
    }
    bail!("Bad input {:?}", input);
}

fn car(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    if let Some(Value::Expr(Edn::List(list))) = input.first() {
        if let Some(res) = list.clone().to_vec().first() {
            return Ok(Value::Expr(res.clone()));
        }
    }
    bail!("Bad input {:?}", input);
}

fn cdr(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    if let Some(Value::Expr(Edn::List(list))) = input.first() {
        if let Some((_, rest)) = list.clone().to_vec().split_first() {
            let res = edn_rs::List::new(rest.to_vec());
            return Ok(Value::Expr(Edn::List(res)));
        }
    }
    bail!("Bad input {:?}", input);
}

fn str_append(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    let mut str = String::new();
    for val in input {
        if let Value::Expr(edn) = val {
            str = match edn {
                Edn::Str(s) => format!("{}{}", str, s),
                other => format!("{}{}", str, other.to_string()),
            }
        } else {
            str = format!("{}{}", str, val);
        }
    }
    Ok(Value::Expr(Edn::Str(str)))
}

fn print_line(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    if let Some(arg) = input.first() {
        match arg {
            Value::Native(_) => println!("#native"),
            Value::Lambda(l) => println!("{:?}", l),
            Value::Expr(e) => println!("{}", e.to_string()),
        }
    }
    Ok(Value::Expr(Edn::Nil))
}

fn read_lisp(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    if let Some(Value::Expr(Edn::Str(lisp))) = input.first() {
        let edn = Edn::from_str(&lisp)?;
        return Ok(Value::Expr(edn))
    }
    bail!("Bad input {:?}", input);
}

pub fn core() -> HashMap<String, Value> {
    let wrap = |it| Value::Native(Native::new(it));
    hashmap! {
        str!("cons")       => wrap(cons),
        str!("car")        => wrap(car),
        str!("cdr")        => wrap(cdr),
        str!("read-str")   => wrap(read_lisp),
        str!("slurp")      => wrap(slurp),
        str!("println")    => wrap(print_line),
        str!("str-append") => wrap(str_append),
    }
}
