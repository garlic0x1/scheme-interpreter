use std::{collections::HashMap, fmt};

use anyhow::{Result, anyhow, bail};
use edn_rs::Edn;
use maplit::hashmap;
use crate::{
    str,
    value::{Value, Lambda},
    evaluator::Evaluator,
};

pub type NativeFn = Box<fn(&[Value], &mut Evaluator) -> Result<Value>>;

#[derive(Clone)]
pub struct Special {
    pub proc: Box<fn(&[Edn], &mut Evaluator) -> Result<Value>>
}

impl Special {
    pub fn new(proc: Box<fn(&[Edn], &mut Evaluator) -> Result<Value>>) -> Self {Self{proc}}
}

impl fmt::Debug for Special {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f, "#special-form")}
}

pub fn special_forms() -> HashMap<String, Special> {
    let wrap = |it| Special::new(Box::new(it));
    hashmap!{
        str!("λ")      => wrap(lambda),
        str!("lambda") => wrap(lambda),
        str!("def")    => wrap(def),
        str!("if")     => wrap(if_statement),
        str!("do")     => wrap(progn),
        str!("quote")  => wrap(quote),
    }
}

fn lambda(input: &[Edn], _env: &mut Evaluator) -> Result<Value> {
    if let (Some(args), Some(body)) = (input.get(0), input.get(1)) {
        if let Edn::Vector(args) = args.clone() {
            return Ok(Value::Lambda(Lambda{
                vars: args.to_vec(),
                body: body.clone(),
            }));
        }
    }
    Err(anyhow!("Invalid args"))
}

fn def(input: &[Edn], env: &mut Evaluator) -> Result<Value> {
   if input.len() % 2 != 0 {
       bail!("Must provide even args");
   }

   input.iter()
       .enumerate()
       .filter(|x| x.0 % 2 == 0)
       .filter_map(|(i, key)|if let (Some(val), key) = (input.get(i + 1), key) {
           Some((key, val))} else {None})
       .for_each(|(key, val)| if let Ok(val) = env.eval(val) {
           env.env.define(&key.to_string(), &val);
       });
   Ok(Value::Expr(Edn::Nil))
}

fn if_statement(input: &[Edn], env: &mut Evaluator) -> Result<Value> {
    if let (Some(pred), Some(then), Some(otherwise)) = (input.get(0), input.get(1), input.get(3)) {
        if let Value::Expr(edn) = env.eval(pred)? {
            match edn {
                Edn::Nil => return env.eval(otherwise),
                _ => return env.eval(then),
            }
        }
    }
    bail!("Bad input {:?}", input);
}

fn quote(input: &[Edn], _env: &mut Evaluator) -> Result<Value> {
    if let Some(first) = input.get(0) {
        return Ok(Value::Expr(first.clone()));
    }
    bail!("Bad input {:?}", input);
}

fn progn(input: &[Edn], env: &mut Evaluator) -> Result<Value> {
    let mut res: Value = Value::Expr(Edn::Nil);
    for expr in input {
        res = env.eval(expr)?;
    }
    Ok(res)
}
