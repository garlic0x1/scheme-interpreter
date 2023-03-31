use std::fmt;
use anyhow::Result;
use edn_rs::Edn;

use crate::evaluator::Evaluator;

#[derive(Clone, Debug)]
pub enum Value {
    Lambda(Lambda),
    Native(Native),
    Expr(Edn),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(l) => write!(f, "{}", l),
            Value::Native(n) => write!(f, "{}", n),
            Value::Expr(e) => write!(f, "{}", e.to_string()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Lambda {
    pub vars: Vec<Edn>,
    pub body: Edn,
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let args = Edn::Vector(edn_rs::Vector::new(self.vars.clone()));
        write!(f, "(λ {} {})", args.to_string(), &self.body.to_string())
    }
}

impl Lambda {
    pub fn new(vars: Vec<Edn>, body: Edn) -> Self {Self{vars, body}}
}

pub type NativeFn = Box<fn(&[Value], &mut Evaluator) -> Result<Value>>;
#[derive(Clone)]
pub struct Native {
    pub proc: NativeFn
}

impl fmt::Debug for Native {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f, "#native")}
}

impl fmt::Display for Native {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f, "#native")}
}

impl Native {
    pub fn new(proc: fn(&[Value], &mut Evaluator) -> Result<Value>) -> Self {Self{proc: Box::new(proc)}}
    pub fn apply(&self, args: &[Value], env: &mut Evaluator) -> Result<Value> {
        (&self.proc)(args, env)}
}
