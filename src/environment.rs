use std::collections::HashMap;
use edn_rs::Edn;
use maplit::hashmap;
use crate::{value::Value, macros::Macro, special::{special_forms, Special}};

pub type Frame = HashMap<String, Value>;
pub type Stack = Vec<Frame>;
pub type MacroTable = HashMap<Edn, Macro>;

#[derive(Debug)]
pub struct Environment {
    stack:  Stack,
    global: Frame,
    special: HashMap<String, Special>,
    macros: MacroTable,
}

impl Environment {
    pub fn new(stack: Stack, global: Frame, macros: MacroTable) -> Self {
        let special = special_forms();
        Self{stack, global, special, macros}
    }

    pub fn from_core(core: Frame) -> Self {
        Environment::new(vec![], core, hashmap!{})
    }

    pub fn get_special(&self, name: &String) -> Option<Special> {
        if let Some(res) = self.special.get(name) {
            Some(res.clone())
        } else {
            None
        }
    }

    pub fn get(&self, name: &String) -> Option<Value> {
        if let Some(stack) = self.stack
                                 .iter()
                                 .rev()
                                 .find_map(|f| f.get(name)) {
            Some(stack.clone())
        } else if let Some(global) = self.global.get(name) {
            Some(global.clone())
        } else {
            None
        }
    }

    pub fn lambda_assign(&mut self, args: &[Value], vars: &[Edn]) {
        self.stack.push(
            vars.iter()
                .zip(args.iter())
                .map(|(key, val)| (key.to_string(), val.clone()))
                .collect::<HashMap<String, Value>>());
    }

    pub fn lambda_pop(&mut self) -> Option<Frame> {self.stack.pop()}

    pub fn define(&mut self, key: &String, val: &Value) {
        self.global.insert(key.clone(), val.clone());
    }

    pub fn defmacro(&mut self, key: &Edn, mac: &Macro) {
        self.macros.insert(key.clone(), mac.clone());
    }
}
