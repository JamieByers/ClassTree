use std::{collections::HashMap, io::{self, Read}};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::structs::FileData;
use serde_json;

pub mod lexer;
pub mod parser;
pub mod structs;


fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);

    let json: serde_json::Value = serde_json::from_str(&buffer).unwrap();
    let all_files = get_all_files(json);

    let mut lexer = Lexer::new(all_files);

    let tkls = lexer.lex();

    let mut parser = Parser::new(tkls);

    let _possible_objects = parser.possible_objects();

    parser.parse();

}

fn get_all_files(json: serde_json::Value) -> Vec<FileData> {
    let mut all_files: Vec<FileData> = Vec::new();

    if let serde_json::Value::Array(files) = &json {
        for file in files {

            let file_no = file["fileNo"].as_i64().unwrap() as i16;
            let filepath = file["fileName"].as_str().unwrap().to_string();
            let file_type = file["fileType"].as_str().unwrap().to_string();
            let mut lines_map = HashMap::new();

           if let serde_json::Value::Array(lines) = &file["lines"] {
                for line in lines {
                    let line_number: i64 = line[0].as_i64().unwrap();
                    let line_body: String = line[1].as_str().unwrap().to_string();
                    lines_map.insert(line_number, line_body);
                }
            }

           let file_data = FileData {
                file_no,
                filepath,
                file_type,
                lines: lines_map,
           };

           all_files.push(file_data);
        }
    }

    return all_files;
}

