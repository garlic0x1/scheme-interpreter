use std::collections::HashMap;
use edn_rs::Edn;
use maplit::hashmap;
use crate::{value::Value, macros::Macro};

pub type Frame = HashMap<Edn, Value>;
pub type Stack = Vec<Frame>;
pub type MacroTable = HashMap<Edn, Macro>;

#[derive(Debug)]
pub struct Environment {
    stack:  Stack,
    global: Frame,
    macros: MacroTable,
}

impl Environment {
    pub fn new(stack: Stack, global: Frame, macros: MacroTable) -> Self {Self{stack, global, macros}}
    pub fn from_core(core: Frame) -> Self {Environment::new(vec![], core, hashmap!{})}

    pub fn get(&self, edn: &Edn) -> Option<Value> {
        if let Some(stack) = self.stack.iter().rev().filter_map(|f| f.get(edn)).next() {
            Some(stack.clone())
        } else if let Some(global) = self.global.get(edn) {
            Some(global.clone())
        } else {
            None
        }
    }

    pub fn lambda_assign(&mut self, args: &[Value], vars: &[Edn]) {
        self.stack.push(
            vars.iter()
                .zip(args.iter())
                .map(|(key, val)| (key.clone(), val.clone()))
                .collect::<HashMap<Edn, Value>>());
    }

    pub fn lambda_pop(&mut self) -> Option<Frame> {self.stack.pop()}

    pub fn define(&mut self, key: &Edn, val: &Value) {
        self.global.insert(key.clone(), val.clone());
    }

    pub fn defmacro(&mut self, key: &Edn, mac: &Macro) {
        self.macros.insert(key.clone(), mac.clone());
    }
}
