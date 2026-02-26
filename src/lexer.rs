use core::panic;
use std::collections::HashMap;
use crate::structs::{FileData, Token, TokenisedFileData, Type};


pub struct Lexer {
    pos: usize,
    current_char: char,
    current_line: String,
    files: Vec<FileData>,
}


impl Lexer {
    pub fn new(files: Vec<FileData>) -> Self {
        Lexer {
            pos: 0,
            current_char: '\0',
            current_line: String::new(),
            files,
        }
    }

    pub fn advance(&mut self) -> char {
        // println!("Current token at advance \"{}\", '{}', {}, {}", self.current_char, self.current_line, self.pos, self.current_line.len());
        if self.pos <= self.current_line.len() {
            self.pos += 1;
            self.current_char = self.current_line[self.pos..]
                .chars()
                .next()
                .unwrap_or('\0');

            self.current_char
        } else {
            '\0'
        }
    }

    pub fn peek(&mut self) -> char {
        if self.pos <= self.current_line.len() {
            let pos = self.pos + 1;
            self.current_char = self.current_line[pos..]
                .chars()
                .next()
                .unwrap_or('\0');

            self.current_char
        } else {
            '\0'
        }

    }

    pub fn lex(&mut self) -> Vec<TokenisedFileData>  {
        let mut tokenised_files = Vec::new();

        for file in self.files.clone() {
            let lines = file.lines.clone();
            let mut new_lines = HashMap::new();

            for line in lines {
                self.current_line = line.1;
                self.pos = 0;
                // println!("{}, {}, {}, {}", self.pos, self.current_char, self.current_line, self.current_line.len());
                self.current_char = self.current_line.chars().nth(self.pos).unwrap_or('\0');

                let line_tokens = self.handle_line();
                // println!("line tokens: {:?}, {}", line_tokens, self.current_line);

                new_lines.insert(line.0, line_tokens);
            }

            let file_no = file.file_no;
            let file_type = file.file_type.clone();
            let filepath = file.filepath.clone();

            let new_file = TokenisedFileData {
                file_no,
                file_type,
                filepath,
                lines: new_lines,
                old_lines: file.lines,

            };

            tokenised_files.push(new_file);
        }

        return tokenised_files

    }

    pub fn handle_line(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.current_char != '\0' {
            let token = self.next_token();
            tokens.push(token);
        }

        return tokens
    }


    pub fn next_token(&mut self) -> Token {
        while self.pos != 0 && self.current_char.is_whitespace() && self.current_char != '\0' {
            self.advance();
        }


        let token = match self.current_char {
            ' ' => self.handle_indent(),

            'a'..='z' | 'A'..='Z' | '_' => self.handle_indentifier(),

            '0'..='9' => self.handle_number(),

            ',' | ':' | ';' | '=' | '.' => self.handle_punctuation(),

            '(' | ')' => self.handle_parenthesis(),

            '[' | ']' => self.handle_brackets(),

            '{' | '}' => self.handle_braces(),

            '<' | '>' => self.handle_angles(),

            '"' | '\'' => self.handle_string(),

            '/' | '#' | '-' => self.handle_comment(),

            '\0' => Token::Eof,

            _ => {
                let c = self.current_char;
                self.advance();
                return Token::Other(c)
            },
        };

        return token
    }

    fn handle_indent(&mut self) -> Token {
        let mut indent_level = 0;

        while self.current_char == ' ' {
            indent_level += 1;
            self.advance();
        }

        Token::Indent(indent_level)

    }

    fn handle_punctuation(&mut self) -> Token {
        let c = match self.current_char {
            '.' => Token::Period,
            ',' => Token::Comma,
            ':' => {
                if self.peek() == ':' {
                    self.advance();
                    Token::Connect
                } else {
                    Token::Colon
                }
            },
            ';' => Token::Semicolon,
            '=' => Token::Equals,
            _ => panic!("Cannot tokenise punctuation")
        };

        self.advance();
        return c
    }

    fn handle_indentifier(&mut self) -> Token {
        let mut identifier: String = String::new();
        while !self.current_char.is_whitespace() && (self.current_char.is_alphabetic() || self.current_char == '_') {
            identifier.push(self.current_char);
            self.advance();
        }

        self.match_identifier(identifier)
    }

    fn match_identifier(&mut self, id: String) -> Token {
        let mut identifier = id.clone();
        identifier.make_ascii_lowercase();

        let object_keywords: Vec<&str> = vec![
            "Class",
            "enum",
            "interface",
            "class",
            "defmodule",
            "defstruct",
            "enum",
            "impl",
            "interface",
            "module",
            "object",
            "protocol",
            "record",
            "struct",
            "table",
            "type",
            "union",
        ];

        if object_keywords.contains(&identifier.as_str()) {
            return Token::ObjectDeclaration
        } else {

            match identifier.as_str() {
                "pub" | "public" => Token::Publicity(true),
                "private" => Token::Publicity(false),
                "self" | "Self" => Token::SelfToken,
                "def" | "fn" => Token::FunctionDeclaration,
                "trait" => Token::Trait,

                "string" | "str" => Token::Type(Type::String),
                "integer" | "int" | "i16" | "i32" | "i64" | "i128" => Token::Type(Type::Integer),
                "float" | "f8" | "f16" | "f32" | "f64" => Token::Type(Type::Float),
                "bool" | "boolean" => Token::Type(Type::Float),
                "char" => Token::Type(Type::Char),
                "none" => Token::Type(Type::NoneType),

                _ => Token::Identifier(id),
            }
        }
    }

    fn handle_number(&mut self) -> Token {
        let mut num = String::new();

        while self.current_char.is_numeric() {
            num.push(self.current_char);
            self.advance();
        }

        Token::Number(num)
    }


    // could be possible issues with strings within strings: \" \" hello world! \" \"
    // --- ADDRESS LATER BEFORE MOVING ONTO PARSER
    fn handle_string(&mut self) -> Token {
        let opening_string = self.current_char;
        self.advance(); // skip "

        let mut string = String::new();
        while self.current_char != opening_string {
            if self.current_char == '\\' {
                string.push(self.current_char);
                self.advance();
            }

            string.push(self.current_char);
            self.advance();
        }

        self.advance(); // skip \"
        Token::StringLiteral(string)
    }

    fn handle_brackets(&mut self) -> Token {
        let bracket = self.current_char;
        self.advance();
        Token::Bracket(bracket)
    }

    fn handle_parenthesis(&mut self) -> Token {
        let paren = self.current_char;
        self.advance(); // skip '(' / ')'
        Token::Parenthesis(paren)
    }

    fn handle_braces(&mut self) -> Token {
        let b = match self.current_char {
            '{' => Token::BlockOpen('{'),
            '}' => Token::BlockClose,
            _ => panic!("Not braces")
        };

        self.advance();
        return b
    }

    fn handle_angles(&mut self) -> Token {
        let angle = self.current_char;
        self.advance();
        Token::AngleBracket(angle)

    }

    fn handle_comment(&mut self) -> Token {
        let mut arrow_flag = false;
        let is_comment: bool = match self.current_char {
            '/' => {
                let next = self.peek();
                if next == '/' || next == '*' || next == '=' {
                    self.advance();
                    true
                } else {
                    false
                }
            },
            '#' => true,
            '-' => {
                if self.peek() == '-' {
                    self.advance();
                    true
                } else if self.peek() == '>' {
                    self.advance();
                    arrow_flag = true;
                    false
                } else {
                    false
                }
            },
            _ => false
        };


        if is_comment {
            let mut comment = String::new();

            while self.current_char != '\0' {
                comment.push(self.current_char);
                self.advance();
            }

            return Token::Comment(comment)
        } else if arrow_flag {
            self.advance();
            return Token::Arrow
        } else {
            let c = self.current_char;
            self.advance();

            return Token::Other(c)
        }
    }
}
