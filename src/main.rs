fn main() {
    println!("Hello, world!");
}

#[test]
fn project_from() {
    const INPUT: &str = r#"
//# [package]
//# name = "test"
//# version = "0.1.0"
//# edition = "2021"
//#
//# [dependencies]
//# rand = "*"

use rand::prelude::*;

fn main() {
    let num: u64 = random();
    println!("num: {}", num);
}
"#;

    const TOML: &str = r#"
[package]
name = "test"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "*"
"#;

    const SRC: &str = r#"
use rand::prelude::*;

fn main() {
    let num: u64 = random();
    println!("num: {}", num);
}
"#;

    let project = Project::from(INPUT);
    assert_eq!(project.toml, TOML.trim());
    assert_eq!(project.src, SRC.trim());
}
