use anyhow::{anyhow, Result, bail};
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
                    res
                },
                Value::Native(proc) => {
                    let res = (&proc.proc)(args, self);
                    res
                },
                Value::Expr(sym) => Err(anyhow!("{} is not a function", sym.to_string()))
            }
        } else {
            Err(anyhow!("{:?} is an empty list", list))
        }
    }

    fn eval_list(&mut self, list: &List) -> Result<Value> {
        if let Some((first, rest)) = list.clone().to_vec().split_first() {
            if let Edn::Symbol(sym) = first {
                if let Some(spec) = self.env.get_special(sym) {
                    let res = (&spec.proc)(rest, self);
                    return res;
                }
            }
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

    // fn handle_recur(&mut self, list: &edn_rs::List) -> Result<Vec<Value>> {
    //     let mut rec: Vec<Value>;
    //     let list = list.clone().to_vec();
    //     if special(list.first().unwrap()) {
    //         rec = list.iter().map(|edn: &Edn| Value::Expr(edn.clone())).collect();
    //         rec[0] = self.eval(list.first().unwrap())?;
    //     } else {
    //         rec = list.iter().filter_map(|it: &Edn| self.eval(it).ok()).collect();
    //     }
    //     Ok(rec)
    // }
}
