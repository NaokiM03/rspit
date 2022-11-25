#[derive(Debug)]
struct Project {
    name: String,
    toml: String,
    src: String,
}

impl From<&str> for Project {
    fn from(src: &str) -> Self {
        let toml = src
            .lines()
            .skip_while(|line| line.is_empty())
            .take_while(|line| line.starts_with("//#"))
            .map(|line| line[3..].trim())
            // .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
            .join("\n");

        let src = src
            .lines()
            .skip_while(|line| line.is_empty() || line.starts_with("//#"))
            .collect::<Vec<&str>>()
            .join("\n");

        let name = {
            let value = toml
                .parse::<toml::Value>()
                .expect("Failed to parse string into toml.");
            value["package"]["name"]
                .as_str()
                .expect("Failed to extract name from toml.")
                .to_owned()
        };

        Project { name, toml, src }
    }
}

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

    const NAME: &str = "test";

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
    assert_eq!(project.name, NAME);
    assert_eq!(project.toml, TOML.trim());
    assert_eq!(project.src, SRC.trim());
}
