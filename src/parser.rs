use anyhow::Result;
use edn_rs::{Edn, EdnError};
use std::str::FromStr;

pub fn read(input: &str) -> Result<Edn, EdnError> {
    Edn::from_str(input)
}

pub fn print(edn: &Edn) -> Result<String> {
    let output = edn.to_string();
    dbg!(&output);
    Ok(output.to_string())
}
