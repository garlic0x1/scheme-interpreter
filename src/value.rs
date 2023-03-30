use std::fmt;

use anyhow::Result;
use edn_rs::Edn;

use crate::{environment::Environment, evaluator::evaluate};

#[derive(Clone, Debug)]
pub enum Value {
    Lambda(Lambda),
    Native(Native),
    Expr(Edn),
}

#[derive(Clone, Debug)]
pub struct Lambda {
    pub vars: Vec<Edn>,
    pub body: Edn,
}

impl Lambda {
    pub fn new(vars: Vec<Edn>, body: Edn) -> Self {Self{vars, body}}
    pub fn apply(&self, args: &[Value], env: &mut Environment) -> Result<Value> {
        env.lambda_assign(args, &self.vars);
        let res = evaluate(&self.body, env);
        env.lambda_pop();
        res
    }
}

pub type NativeFn = Box<fn(&[Value], &mut Environment) -> Result<Value>>;
#[derive(Clone)]
pub struct Native {
    pub proc: NativeFn
}

impl fmt::Debug for Native {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f, "#native")}
}

impl Native {
    pub fn new(proc: fn(&[Value], &mut Environment) -> Result<Value>) -> Self {Self{proc: Box::new(proc)}}
    pub fn apply(&self, args: &[Value], env: &mut Environment) -> Result<Value> {(&self.proc)(args, env)}
}
