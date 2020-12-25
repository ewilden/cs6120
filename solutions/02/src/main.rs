use bril_rs::*;
use std::io::*;
use serde_json::Result;
use serde::{Deserialize, Serialize};

fn main() -> std::io::Result<()> {
    let mut raw_inp = String::new();
    std::io::stdin().read_to_string(&mut raw_inp)?;
    let pgm: Program = serde_json::from_str(&raw_inp)?;
    println!("{}", serde_json::to_string_pretty(&pgm)?);
    Ok(())
}
