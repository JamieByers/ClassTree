use core::panic;

#[derive(Debug, PartialEq)]
pub enum Token {
    ObjectDeclaration,
    Identifier(String),
    StringLiteral(String),
    Number(String),
    BlockOpen(char),
    BlockClose,
    Comma,
    Semicolon,
    Parenthesis(char),
    Equals,
    Other(char),
    Eof,
}

pub struct Lexer {
    pos: usize,
    current_char: char,
    current_line: String,
    lines: Vec<(i64, String, String)>,
}


impl Lexer {
    pub fn new(lines: Vec<(i64, String, String)>) -> Self {
        let first_line = lines[0].1.clone();
        let c = first_line.chars().next();

        Lexer {
            pos: 0,
            current_char: c.expect("No initial char"),
            current_line: first_line,
            lines,
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

    pub fn lex(&mut self) -> Vec<(i64, Vec<Token>, String)>  {
        let mut tokenised_lines: Vec<(i64, Vec<Token>, String)> = Vec::new();

        for line in self.lines.clone() {
            self.current_line = line.1;
            self.pos = 0;
            self.current_char = self.current_line.chars().nth(self.pos).unwrap_or('\0');

            let line_tokens = self.handle_line();
            // println!("line tokens: {:?}, {}", line_tokens, self.current_line);

            tokenised_lines.push((line.0, line_tokens, line.2));
        }

        return tokenised_lines

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
        while self.current_char.is_whitespace() && self.current_char != '\0' {
            self.advance();
        }


        let token = match self.current_char {
            'a'..='z' | 'A'..='Z' => {
                let identifier = self.handle_indentifier();
                let identifier_token = self.match_identifier(identifier);
                // dont advance again in these functions

                identifier_token
            },

            '0'..='9' => self.handle_number(),

            ',' | ':' | ';' | '=' => self.handle_punctuation(),

            '(' | ')' => self.handle_parenthesis(),

            '{' | '}' => self.handle_braces(),

            '"' | '\'' => self.handle_string(),

            '\0' => Token::Eof,

            _ => {
                let c = self.current_char;
                self.advance();
                return Token::Other(c)
            },
        };

        return token
    }

    fn handle_punctuation(&mut self) -> Token {
        let c = match self.current_char {
            ',' => Token::Comma,
            ':' => Token::BlockOpen(':'),
            ';' => Token::Semicolon,
            '=' => Token::Equals,
            _ => panic!("Cannot tokenise punctuation")
        };

        self.advance();
        return c
    }

    fn handle_indentifier(&mut self) -> String{
        let mut identifier: String = String::new();
        while !self.current_char.is_whitespace() && (self.current_char.is_alphabetic() || self.current_char == '_') {
            identifier.push(self.current_char);
            self.advance();
        }

        identifier
    }

    fn match_identifier(&mut self, identifier: String) -> Token {
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
            "trait",
            "type",
            "union",
        ];

        if object_keywords.contains(&identifier.as_str()) {
            return Token::ObjectDeclaration
        } else {
            return Token::Identifier(identifier);
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


    // could be possible issues with strings within strings: " \" hello world! \" "
    // --- ADDRESS LATER BEFORE MOVING ONTO PARSER
    fn handle_string(&mut self) -> Token {
        let opening_string = self.current_char;
        self.advance(); // skip "

        let mut string = String::new();
        while self.current_char != opening_string {
            string.push(self.current_char);
            self.advance();
        }

        self.advance(); // skip "
        Token::StringLiteral(string)
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
}
