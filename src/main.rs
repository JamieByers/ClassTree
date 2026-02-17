use std::{io::{self, Read}};
use crate::lexer::Lexer;
use crate::parser::Parser;
use serde_json;

pub mod lexer;
pub mod parser;

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);

    let json: serde_json::Value = serde_json::from_str(&buffer).unwrap();
    let all_lines = get_all_lines(json);

    // for line in all_lines.clone() {
    //     println!("{}", line.1);
    // }

    // let possible_objects = find_possible_objects(all_lines);

    let mut lexer = Lexer::new(all_lines);

    let tkls = lexer.lex();

    let mut parser = Parser::new(tkls);

    let _possible_objects = parser.possible_objects();

}

fn find_possible_objects(lines: Vec<(i64, String, String)>) -> Vec<(i64, String, String)> {
    let object_keywords: Vec<&str> = vec![
        "Class",
        "enum",
        "interface",
        "module",
        "structure",
        "class",
        "contract",
        "data",
        "defmodule",
        "defstruct",
        "enum",
        "enum class",
        "impl",
        "interface",
        "library",
        "mixin",
        "module",
        "newtype",
        "object",
        "protocol",
        "record",
        "struct",
        "table",
        "trait",
        "type",
        "union",
    ];

    let mut possible_objects = Vec::new();
    for line in lines {
        let line_no = line.0.clone();
        let line_body = line.1.clone();
        let file = line.2.clone();

        let mut split_line = line.1.split(" ");
        while let Some(next) = split_line.next() {
            let trimmed_next = next.trim().trim_matches('"');

            if next != "" && object_keywords.contains(&trimmed_next) {
                possible_objects.push((line_no, line_body.clone(), file.clone()));
            }
        }
    }

    println!("Classes: {:?}", possible_objects);
    return possible_objects
}

fn get_all_lines(json: serde_json::Value) -> Vec<(i64, String, String)> {
    let mut all_lines: Vec<(i64, String, String)> = Vec::new();

    if let serde_json::Value::Array(files) = &json {
        for file in files {
            let file_name = file["fileName"].as_str().unwrap().to_string();
           if let serde_json::Value::Array(lines) = &file["lines"] {
                for line in lines {
                    let line_body: String = line[1].as_str().unwrap().to_string();
                    let line_number: i64 = line[0].as_i64().unwrap();
                    all_lines.push((line_number, line_body, file_name.clone()));
                }
            }
        }
    }

    return all_lines;
}

