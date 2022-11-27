//# [package]
//# name = "rand"
//# version = "0.1.0"
//# edition = "2021"
//#
//# [dependencies]
//# rand = "*"
//#
//# [profile.release]
//# lto = true

use rand::prelude::*;

fn main() {
    let num: u64 = random();
    println!("num: {}", num);
}

//# ---

//# [package]
//# name = "json"
//# version = "0.1.0"
//# edition = "2021"
//#
//# [dependencies]
//# serde_json = "*"
//#
//# [profile.release]
//# lto = true

use serde_json::{Result, Value};

fn main() -> Result<()> {
    let json = r#"
{
    "name": "Alice",
    "age": 42
}
"#;
    let json: Value = serde_json::from_str(json)?;
    println!("name: {}, age: {}", json["name"], json["age"]);

    Ok(())
}
