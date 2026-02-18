use crate::HashMap;

#[derive(Clone)]
pub struct FileData {
    pub file_no: i16,
    pub file_type: String,
    pub filepath: String,
    pub lines: HashMap<i64, String>
}

#[derive(Debug)]
pub struct TokenisedFileData {
    pub file_no: i16,
    pub file_type: String,
    pub filepath: String,
    pub old_lines: HashMap<i64, String>,
    pub lines: HashMap<i64, Vec<Token>>

}

#[derive(Debug, PartialEq)]
pub enum Token {
    ObjectDeclaration,
    Publicity(bool),
    Identifier(String),
    StringLiteral(String),
    Number(String),
    Comment(String),
    BlockOpen(char),
    BlockClose,
    Comma,
    Semicolon,
    Parenthesis(char),
    Equals,
    Other(char),
    Eof,
}
