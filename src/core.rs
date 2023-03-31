use crate::{value::*, environment::Environment, evaluator::Evaluator};
use anyhow::{anyhow, Result, bail};
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

    // let if_statement = Native::new(|input: &[Value], env: &mut Environment| -> Result<Value> {
    //     if let Some(Value::Expr(Edn::Bool(pred))) = input.get(0) {

    //     }
    // };


    let str_append = Native::new(|input: &[Value], env: &mut Evaluator| -> Result<Value> {
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
    });

    let define = Native::new(|input: &[Value], env: &mut Evaluator| -> Result<Value> {
        //dbg!("gothere", input);
        if input.len() % 2 != 0 {
            dbg!("Must provide even args");
            bail!("Must provide even args");
        }

        input.iter()
            .enumerate()
            .filter(|x| x.0 % 2 == 0)
            .filter_map(|(i, key)| if let (Some(Value::Expr(val)), Value::Expr(key)) = (input.get(i + 1), key) {
                Some((key, val))} else {None})
            .for_each(|(key, val)| if let Ok(val) = env.eval(val) {
                env.env.define(key, &val);
            });
        // dbg!(&env.env);
        Ok(Value::Expr(Edn::Nil))
    });

    let lambda = Native::new(|input: &[Value], _env: &mut Evaluator| -> Result<Value> {
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

    let println = Native::new(|input: &[Value], _env: &mut Evaluator| -> Result<Value> {
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
        Edn::Symbol(str!("define")) => Value::Native(define),
        Edn::Symbol(str!("str-append")) => Value::Native(str_append),
        // Edn::Symbol(str!("cons")) => Value::Proc(cons),
        // Edn::Symbol(str!("car"))  => Value::Proc(car),
        // Edn::Symbol(str!("cdr"))  => Value::Proc(cdr),
    }
}
