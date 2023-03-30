use anyhow::{anyhow, Result, bail};
use edn_rs::Edn;
use crate::{environment::Environment, value::Value};

fn special(expr: &Edn) -> bool {
    if expr.to_string() == "lambda".to_string() {
        // dbg!("SPECIAL");
        true
    } else {
        false
    }
}

fn handle_recur(list: &edn_rs::List, env: &mut Environment) -> Result<Vec<Value>> {
    let mut rec: Vec<Value>;
    let list = list.clone().to_vec();
    if special(list.first().unwrap()) {
        rec = list.iter().map(|edn: &Edn| Value::Expr(edn.clone())).collect();
        rec[0] = evaluate(list.first().unwrap(), env)?;
    } else {
        rec = list.iter().filter_map(|it: &Edn| {
            let r = evaluate(it, env);
            dbg!(&r);
            r.ok()
        }).collect();
        // if rec.len() == list.to_vec().len() {
        //     dbg!("fail");
        //     bail!("failed to evaluate children")
        // }
    }
    Ok(rec)
}

pub fn evaluate(expr: &Edn, env: &mut Environment) -> Result<Value> {
    dbg!(expr);
    match &expr {
        Edn::List(list) => {
            if let Some((key, args)) = handle_recur(list, env)?.split_first() {
                dbg!(key);
                match key {
                    Value::Lambda(lamb) => lamb.apply(args, env),
                    Value::Native(proc) => proc.apply(args, env),
                    Value::Expr(sym) => Err(anyhow!("{} is not a function", sym.to_string()))
                }
            } else {
                Err(anyhow!("{} is an empty list", list.to_string()))
            }
        },
        Edn::Symbol(_sym) => {
            if let Some(val) = env.get(expr) {
                Ok(val)
            } else {
                Ok(Value::Expr(Edn::Nil))
            }
        },
        _ => Ok(Value::Expr(expr.clone()))
    }
}
