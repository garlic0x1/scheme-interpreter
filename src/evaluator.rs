use anyhow::{Result, bail};
use edn_rs::{Edn, List};
use crate::{environment::{Environment, Frame}, value::*};

pub struct Evaluator {
    pub env: Environment,
}

impl Evaluator {
    pub fn from_core(core: Frame) -> Self {
        Self{env: Environment::from_core(core)}
    }

    pub fn apply(&mut self, list: &[Value]) -> Result<Value> {
        if let Some((key, args)) = list.split_first() {
            match key {
                Value::Lambda(lamb) => {
                    self.env.lambda_assign(args, &lamb.vars);
                    let res = self.eval(&lamb.body);
                    self.env.lambda_pop();
                    return res;
                },
                Value::Native(proc) => {
                    let res = (&proc.proc)(args, self);
                    return res;
                },
                Value::Expr(sym) => bail!("{} is not a function", sym.to_string())
            }
        }
        bail!("{:?} is an empty list", list)
    }

    fn eval_list(&mut self, list: &List) -> Result<Value> {
        if let Some((first, rest)) = list.clone().to_vec().split_first() {
            // special forms
            if let Edn::Symbol(sym) = first {
                if let Some(spec) = self.env.get_special(sym) {
                    let res = (&spec.proc)(rest, self);
                    return res;
                }
            }
            // clojure style maps
            if let (Edn::Key(key), Some(map)) = (first, rest.first()) {
                let val = self.eval(map)?;
                if let Value::Expr(Edn::Map(map)) = val {
                    let btree = map.to_map();
                    let res = btree.get(key).unwrap_or(&Edn::Nil);
                    return Ok(Value::Expr(res.clone()));
                }
            }
            // regular function
            let new = list.clone().to_vec().iter()
                          .filter_map(|it: &Edn| self.eval(it).ok())
                          .collect::<Vec<Value>>();
            return self.apply(&new)
        }
        bail!("Empty list");
    }

    fn eval_symbol(&mut self, sym: &String) -> Result<Value> {
        if let Some(val) = self.env.get(sym) {
            return Ok(val)
        }
        Ok(Value::Expr(Edn::Nil))
    }

    pub fn eval(&mut self, expr: &Edn) -> Result<Value> {
        match &expr {
            Edn::List(list) => self.eval_list(list),
            Edn::Symbol(sym) => self.eval_symbol(sym),
            _ => Ok(Value::Expr(expr.clone()))
        }
    }
}
