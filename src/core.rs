use crate::{value::*, evaluator::Evaluator};
use anyhow::{Result, bail};
use edn_rs::Edn;
use maplit::hashmap;
use std::{collections::HashMap, str::FromStr};
use std::convert::TryFrom;

#[macro_export]
macro_rules! str {
    () => {
        String::new()
    };
    ($x:expr $(,)?) => {
        ToString::to_string(&$x)
    };
}

fn add(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    let mut sum_int: i64   = 0;
    let mut sum_float: f64 = 0.0;
    for val in input.iter() {
        if let Value::Expr(edn) = val {
            match &edn {
                Edn::UInt(int) => sum_int += i64::try_from(*int)?,
                Edn::Int(int)  => sum_int += i64::try_from(*int)?,
                Edn::Double(_)   => sum_float += edn.to_float().unwrap_or_default(),
                Edn::Rational(_) => sum_float += edn.to_float().unwrap_or_default(),
                _ => bail!("Bad value {:?}", edn)
            }
        }
    }
    if sum_float == 0.0 {
        Ok(Value::Expr(Edn::Int(sum_int as isize)))
    } else {
        Ok(Value::Expr(Edn::Double(edn_rs::Double::from(sum_float + sum_int as f64))))
    }
}

fn multiply(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    let mut sum_int: i64   = 1;
    let mut sum_float: f64 = 1.0;
    for val in input.iter() {
        if let Value::Expr(edn) = val {
            match &edn {
                Edn::UInt(int)   => sum_int   *= i64::try_from(*int)?,
                Edn::Int(int)    => sum_int   *= i64::try_from(*int)?,
                Edn::Double(_)   => sum_float *= edn.to_float().unwrap_or_default(),
                Edn::Rational(_) => sum_float *= edn.to_float().unwrap_or_default(),
                _ => bail!("Bad value {:?}", edn)
            }
        }
    }
    if sum_float == 1.0 {
        Ok(Value::Expr(Edn::Int(sum_int as isize)))
    } else {
        Ok(Value::Expr(Edn::Double(edn_rs::Double::from(sum_float * sum_int as f64))))
    }
}

fn divide(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    if let (Some(Value::Expr(numer)), Some(Value::Expr(denom))) = (input.get(0), input.get(1)) {
        return Ok(Value::Expr(Edn::Double(edn_rs::Double::from(
            numer.to_float().unwrap_or_default() /
                denom.to_float().unwrap_or_default()))));
    }
    bail!("Bad input {:?}", input);
}

fn slurp(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    if let Some(Value::Expr(Edn::Str(filename))) = input.first() {
        let text = std::fs::read_to_string(filename)?;
        return Ok(Value::Expr(Edn::Str(text)));
    }
    bail!("Bad input {:?}", input);
}

fn conj(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    if let (Some(Value::Expr(Edn::Map(a))), Some(Value::Expr(Edn::Map(b)))) = (input.get(0), input.get(1)) {
        let mut first  = a.clone().to_map();
        let second = b.clone().to_map();
        for (key, val) in second {
            first.insert(key, val);
        }
        let new = edn_rs::Map::new(first);
        return Ok(Value::Expr(Edn::Map(new)));
    }
    bail!("Bad input {:?}", input);
}
fn cons(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    if let (Some(car), Some(cdr)) = (input.get(0), input.get(1)) {
        if let (Value::Expr(car), Value::Expr(cdr)) = (car, cdr) {
            if let Edn::List(list) = cdr {
                let mut v = list.clone().to_vec();
                v.insert(0, car.clone());
                let res = edn_rs::List::new(v);
                return Ok(Value::Expr(Edn::List(res)));
            }
        }
    }
    bail!("Bad input {:?}", input);
}

fn car(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    if let Some(Value::Expr(Edn::List(list))) = input.first() {
        if let Some(res) = list.clone().to_vec().first() {
            return Ok(Value::Expr(res.clone()));
        }
    }
    bail!("Bad input {:?}", input);
}

fn cdr(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    if let Some(Value::Expr(Edn::List(list))) = input.first() {
        if let Some((_, rest)) = list.clone().to_vec().split_first() {
            let res = edn_rs::List::new(rest.to_vec());
            return Ok(Value::Expr(Edn::List(res)));
        }
    }
    bail!("Bad input {:?}", input);
}

fn str_append(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
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
}

fn print_line(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    if let Some(arg) = input.first() {
        match arg {
            Value::Native(_) => println!("#native"),
            Value::Lambda(l) => println!("{:?}", l),
            Value::Expr(e) => println!("{}", e.to_string()),
        }
    }
    Ok(Value::Expr(Edn::Nil))
}

fn read_lisp(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    if let Some(Value::Expr(Edn::Str(lisp))) = input.first() {
        let edn = Edn::from_str(&lisp)?;
        return Ok(Value::Expr(edn))
    }
    bail!("Bad input {:?}", input);
}

fn equals_edn(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    if let (Some(Value::Expr(a)), Some(Value::Expr(b))) = (input.get(0), input.get(1)) {
        if a == b {
            return Ok(Value::Expr(Edn::Bool(true)));
        } else {
            return Ok(Value::Expr(Edn::Bool(false)));
        }
    }
    bail!("Bad input {:?}", input);
}

fn type_of(input: &[Value], _env: &mut Evaluator) -> Result<Value> {
    if let Some(val) = input.first() {
        return Ok(Value::Expr(Edn::Str(
            match val {
                Value::Lambda(_) => str!("Lambda"),
                Value::Native(_) => str!("Native"),
                Value::Expr(edn) => match edn {
                    Edn::Tagged(..)  => str!("Tagged"),
                    Edn::Vector(_)   => str!("Vector"),
                    Edn::Set(_)      => str!("Set"),
                    Edn::Map(_)      => str!("Map"),
                    Edn::List(_)     => str!("List"),
                    Edn::Key(_)      => str!("Key"),
                    Edn::Symbol(_)   => str!("Symbol"),
                    Edn::Str(_)      => str!("Str"),
                    Edn::Int(_)      => str!("Int"),
                    Edn::UInt(_)     => str!("Int"),
                    Edn::Double(_)   => str!("Float"),
                    Edn::Rational(_) => str!("Rational"),
                    Edn::Char(_)     => str!("Char"),
                    Edn::Bool(_)     => str!("Bool"),
                    Edn::Inst(_)     => str!("Inst"),
                    Edn::Uuid(_)     => str!("Uuid"),
                    Edn::Nil         => str!("Nil"),
                    Edn::Empty       => str!("Empty"),
                    Edn::NamespacedMap(..) => str!("NamespacedMap"),
                }
            }
        )));
    }
    bail!("Bad input {:?}", input);
}

pub fn core() -> HashMap<String, Value> {
    let wrap = |it| Value::Native(Native::new(it));
    hashmap! {
        str!("true") => Value::Expr(Edn::Bool(true)),
        str!("false") => Value::Expr(Edn::Bool(false)),
        str!("+")          => wrap(add),
        str!("*")          => wrap(multiply),
        str!("/")          => wrap(divide),
        str!("type")       => wrap(type_of),
        str!("conj")       => wrap(conj),
        str!("cons")       => wrap(cons),
        str!("car")        => wrap(car),
        str!("cdr")        => wrap(cdr),
        str!("read-str")   => wrap(read_lisp),
        str!("slurp")      => wrap(slurp),
        str!("println")    => wrap(print_line),
        str!("str-append") => wrap(str_append),
        str!("=")          => wrap(equals_edn),
    }
}
