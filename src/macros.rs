use anyhow::{Result, anyhow};
use edn_rs::Edn;

#[derive(Clone, Debug)]
pub struct Macro {vars: Vec<Edn>, body: Edn}

impl Macro {
    pub fn macro_expand(&self) -> Result<Edn> {
        let _ = self.vars;
        let _ = self.body;
        Err(anyhow!("todo"))
    }
}
