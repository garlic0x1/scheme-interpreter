use std::collections::HashMap;
//use crate::list::List;
use anyhow::{anyhow, Result};
use edn_rs::Edn;

pub type Frame = HashMap<Edn, Value>;
pub type Stack = Vec<Frame>;
pub type Procedure = Box<fn(&[Value]) -> Result<Value>>;

#[derive(Clone)]
pub enum Value {
    Lambda(Lambda),
    Proc(Procedure),
    Expr(Edn),
}

#[derive(Clone, Debug)]
pub struct Lambda {
    pub vars: Vec<Edn>,
    pub body: Edn,
}

impl Lambda {
    pub fn new(vars: Vec<Edn>, body: Edn) -> Result<Self> {
        if !vars.iter().all(|a| if let Edn::Symbol(_) = a {true} else {false}) {
            Err(anyhow!("Var names must be symbols"))
        } else {
            Ok(Self{vars, body})
        }
    }

    pub fn frame(&self, args: &[Value]) -> Frame {
        self.vars
            .iter()
            .zip(args.iter())
            .map(|(key, val)| (key.clone(), val.clone()))
            .collect::<HashMap<Edn, Value>>()
    }

    pub fn eval(&self, args: &[Value], stack: &mut Stack) -> Result<Value> {
        stack.push(self.frame(args));
        let res = evaluate(&self.body, stack);
        stack.pop();
        res
    }
}

pub fn apply_lambda(proc: &Lambda, args: &[Value], stack: &mut Stack) -> Result<Value> {
    // dbg!("applying lambda {:?}", proc);
    proc.eval(args, stack)
}

pub fn apply_proc(proc: &Procedure, args: &[Value], _stack: &Stack) -> Result<Value> {
    //dbg!("applying proc {}");
    proc(args)
}

fn special(expr: &Edn) -> bool {
    if expr.to_string() == "lambda".to_string() {
        // dbg!("SPECIAL");
        true
    } else {
        false
    }
}

pub fn evaluate(expr: &Edn, stack: &mut Stack) -> Result<Value> {
    let mut stack = stack;
    match &expr {
        Edn::List(list) => {
            let rec: Vec<Value>;
            if special(list.clone().to_vec().first().unwrap()) {
                rec = list
                    .clone()
                    .to_vec()
                    .iter()
                    .map(|edn: &Edn| Value::Expr(edn.clone()))
                    .collect();
            } else {
                rec = list
                    .clone()
                    .to_vec()
                    .iter()
                    .filter_map(|it: &Edn| evaluate(it, &mut stack).ok())
                    .collect();
            }

            //dbg!("evaluated children");

            if let Some((first, rest)) = rec.split_first() {
                match first {
                    Value::Lambda(lamb) => apply_lambda(lamb, rest, &mut stack),
                    Value::Proc(proc) =>   apply_proc(proc, rest, stack),
                    Value::Expr(sym) => {
                        if let Some(val) = get_value(stack, sym) {
                            match val {
                                Value::Lambda(lamb) => apply_lambda(&lamb, rest, &mut stack),
                                Value::Proc(proc) =>   apply_proc(&proc, rest, stack),
                                Value::Expr(err) => Err(anyhow!("{} is not a function", err))
                            }
                        } else {
                            Ok(Value::Expr(Edn::Nil))
                        }

                    }
                }
            } else {
                Ok(Value::Expr(Edn::Nil))
            }
        },
        Edn::Symbol(_sym) => {
            if let Some(val) = get_value(stack, expr) {
                Ok(val)
            } else {
                Ok(Value::Expr(Edn::Nil))
            }
        },
        _ => Ok(Value::Expr(expr.clone()))
    }
}

pub fn get_value(stack: &Stack, name: &Edn) -> Option<Value> {
    for frame in stack.iter() {
        for (key, val) in frame.iter() {
            if key == name {
                return Some(val.clone());
            }
        }
    }
    None
}

// fn vec_cons<T>(it: T, coll: &mut Vec<T>) -> &Vec<T> {
//     coll.push(it);
//     coll
// }

// fn vec_car<T>(coll: &mut Vec<T>) -> Option<T> {
//     coll.pop()
// }

// fn vec_cdr<T>(coll: &mut Vec<T>) -> &Vec<T> {
//     coll.pop();
//     coll
// }
