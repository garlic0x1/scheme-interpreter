use crate::{value::*, environment::Environment};
use anyhow::{anyhow, Result};
use edn_rs::Edn;
use maplit::hashmap;
use std::collections::HashMap;

macro_rules! str {
    () => {
        String::new()
    };
    ($x:expr $(,)?) => {
        ToString::to_string(&$x)
    };
}

pub fn core() -> HashMap<Edn, Value> {
    // let cons: Procedure = |input| {
    //     if let (Some(car), cdr) = (input.head(), input.tail()) {
    //         let pair = cdr.cons(car.clone());
    //         Ok(Expr::List(pair))
    //     } else {
    //         Err(anyhow!("Bad input"))
    //     }
    // };

    // let car: Procedure = |input| {
    //     if let Some(Expr::List(list)) = input.head() {
    //         if let Some(car) = list.head() {
    //             Ok(car.clone())
    //         } else {
    //             Err(anyhow!("empty input"))
    //         }
    //     } else {
    //         Err(anyhow!("Bad input"))
    //     }
    // };

    // let cdr: Procedure = |input| {
    //     if let Some(Expr::List(list)) = input.head() {
    //         Ok(Expr::List(list.tail()))
    //     } else {
    //         Err(anyhow!("Bad input"))
    //     }
    // };
    // let apply: Procedure = Box::new(|input: &[Value]| -> Resut<Value>)

    let lambda = Native::new(|input: &[Value], _env: &mut Environment| -> Result<Value> {
        if let (Some(first), Some(rest)) = (input.get(0), input.get(1)) {
            if let (Value::Expr(args), Value::Expr(body)) = (first, rest) {
                if let Edn::Vector(args) = args.clone() {
                    return Ok(Value::Lambda(Lambda{
                        vars: args.to_vec(),
                        body: body.clone(),
                    }));
                }
            }
        }
        Err(anyhow!("Invalid args"))
    });

    let println = Native::new(|input: &[Value], _env: &mut Environment| -> Result<Value> {
        dbg!("gothere");
        if let Some(arg) = input.first() {
            match arg {
                Value::Native(_) => println!("#native"),
                Value::Lambda(l) => println!("{:?}", l),
                Value::Expr(e) => println!("{}", e.to_string()),
            }
        }
        Ok(Value::Expr(Edn::Nil))
    });

    hashmap! {
        Edn::Symbol(str!("println")) => Value::Native(println),
        Edn::Symbol(str!("lambda")) => Value::Native(lambda),
        // Edn::Symbol(str!("cons")) => Value::Proc(cons),
        // Edn::Symbol(str!("car"))  => Value::Proc(car),
        // Edn::Symbol(str!("cdr"))  => Value::Proc(cdr),
    }
}
