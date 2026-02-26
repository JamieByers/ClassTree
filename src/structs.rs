use crate::HashMap;

#[derive(Clone)]
pub struct FileData {
    pub file_no: i16,
    pub file_type: String,
    pub filepath: String,
    pub lines: HashMap<i64, String>
}

#[derive(Debug, Clone)]
pub struct TokenisedFileData {
    pub file_no: i16,
    pub file_type: String,
    pub filepath: String,
    pub old_lines: HashMap<i64, String>,
    pub lines: HashMap<i64, Vec<Token>>

}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Object(Object),
    Variable(Variable),
    Function(Function),
    None,
    Eol,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub identifier: String,
    pub value: Option<Vec<Token>>,
    pub vtype: Option<Vec<Token>>,
    pub parent: String,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub identifier: String,
    pub ptype: Option<String>,

}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub identifier: String,
    pub block: Vec<ASTNode>,
    pub public: bool,
    pub variables: HashMap<String, ASTNode>,

    pub parent: Option<Box<Object>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub identifier: String,
    pub parameters: Vec<Parameter>,
    pub return_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Char,
    String,
    Integer,
    Float,
    Boolean,
    Object,
    NoneType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    ObjectDeclaration,
    Trait,
    VariableDeclaration,
    SelfToken,
    FunctionDeclaration,
    Publicity(bool),
    Identifier(String),
    StringLiteral(String),
    Number(String),
    Type(Type),
    Comment(String),
    Indent(i16),
    BlockOpen(char),
    BlockClose,
    Bracket(char),
    Period,
    Comma,
    Colon,
    Connect,
    Semicolon,
    Arrow,
    Parenthesis(char),
    AngleBracket(char),
    Equals,
    Other(char),

    None,
    Eof,
}

impl Token {
    pub fn to_string(self) -> String {
        match self {
            Token::ObjectDeclaration => String::from("Object"),
            Token::VariableDeclaration => format!("var"),
            Token::SelfToken => format!("self"),
            Token::FunctionDeclaration => format!("Function"),
            Token::Publicity(bool) => {
                if bool {
                    String::from("pub")
                } else {
                    String::from("private")
                }
            },
            Token::Identifier(s) => s,
            Token::StringLiteral(s) => s,
            Token::Number(i) => i,
            Token::Comment(s) => s,
            Token::Indent(_i) => String::from("Indent"),
            Token::BlockOpen(c) => format!("{}", c),
            Token::BlockClose => String::from("}"),
            Token::Period => String::from("."),
            Token::Comma => String::from(","),
            Token::Colon => String::from(":"),
            Token::Semicolon => String::from(";"),
            Token::Arrow => String::from("->"),
            Token::Parenthesis(p) => format!("{}", p),
            Token::Equals => String::from("="),
            Token::Other(o) => String::from(o),
            Token::Eof => String::from("Eof"),

            _ => format!("{:?}", self)
        }
    }
}
