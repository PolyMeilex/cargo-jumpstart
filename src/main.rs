use clap::{App, Arg};

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let matches = App::new("cargo-jumpstart")
        .version("0.1.0")
        .author("Poly <marynczak.bartlomiej@gmail.com>")
        .about("Util for rust file scaffolding")
        .subcommand(
            App::new("object")
                .about("Create rust file with object inside")
                .arg(Arg::with_name("name")),
        )
        .get_matches();

    if let Some(c) = matches.subcommand_matches("object") {
        if let Some(o) = c.value_of("name") {
            let file_str = format!(
                r#"
pub struct {}{{
}}

impl {}{{
    pub fn new() -> Self{{
        Self{{}}
    }}
}}
                "#,
                o, o
            );

            let chars: Vec<char> = o.chars().collect();
            let mut words: Vec<Vec<char>> = Vec::new();

            for c in chars {
                if c.is_uppercase() {
                    words.push(Vec::new());
                }

                let id = words.len() - 1;
                words[id].push(c);
            }

            let words: Vec<String> = words
                .into_iter()
                .map(|w| w.into_iter().collect::<String>().to_lowercase())
                .collect();
            let file_name = format!("{}.rs", words.join("_"));

            let mut file = File::create(&file_name).unwrap();
            file.write(file_str.as_bytes()).unwrap();
        }
    }
}
