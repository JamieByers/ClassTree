use crate::structs::{Token, TokenisedFileData};

pub struct Parser {
   files: Vec<TokenisedFileData>,
}

impl Parser {
    pub fn new(files: Vec<TokenisedFileData>) -> Parser {
        Parser {
            files
        }
    }

    pub fn possible_objects(&mut self) {
        for file in &self.files {
            for lines in &file.lines {
                if lines.1.contains(&Token::ObjectDeclaration) {
                    println!("{} {:?} {:?}", lines.0, lines.1, file.filepath)
                }
            }
        }
    }
}
