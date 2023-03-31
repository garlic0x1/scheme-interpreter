use anyhow::{anyhow, Result};
use edn_rs::Edn;
use crate::{environment::{Environment, Frame}, value::Value};

pub struct Evaluator {
    pub env: Environment,
}

fn special(expr: &Edn) -> bool {
    let specials = ["lambda", "define"];
    specials.contains(&expr.to_string().as_str())
}

impl Evaluator {
    pub fn from_core(core: Frame) -> Self {Self{env: Environment::from_core(core)}}

    pub fn apply(&mut self, list: &[Value]) -> Result<Value> {
        if let Some((key, args)) = list.split_first() {
            match key {
                Value::Lambda(lamb) => {
                    // dbg!("applying");
                    self.env.lambda_assign(args, &lamb.vars);
                    let res = self.eval(&lamb.body);
                    self.env.lambda_pop();
                    res
                },
                Value::Native(proc) => {
                    let res = (&proc.proc)(args, self);
                    // dbg!(&res);
                    res
                },
                Value::Expr(sym) => Err(anyhow!("{} is not a function", sym.to_string()))
            }
        } else {
            Err(anyhow!("{:?} is an empty list", list))
        }
    }

    pub fn eval(&mut self, expr: &Edn) -> Result<Value> {
        // dbg!(&expr);
        match &expr {
            Edn::List(list) => {
                let new = self.handle_recur(list)?;
                // dbg!(list);
                // dbg!(&new);
                self.apply(&new)
            },
            Edn::Symbol(_sym) => {
                if let Some(val) = self.env.get(expr) {
                    Ok(val)
                } else {
                    Ok(Value::Expr(Edn::Nil))
                }
            },
            _ => Ok(Value::Expr(expr.clone()))
        }
    }

    fn handle_recur(&mut self, list: &edn_rs::List) -> Result<Vec<Value>> {
        let mut rec: Vec<Value>;
        let list = list.clone().to_vec();
        if special(list.first().unwrap()) {
            rec = list.iter().map(|edn: &Edn| Value::Expr(edn.clone())).collect();
            rec[0] = self.eval(list.first().unwrap())?;
        } else {
            rec = list.iter().filter_map(|it: &Edn| self.eval(it).ok()).collect();
        }
        Ok(rec)
    }
}
