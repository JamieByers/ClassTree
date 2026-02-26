use std::collections::HashMap;

use crate::structs::{ASTNode, Function, Object, Parameter, Token, TokenisedFileData, Variable};

pub struct Parser {
    files: Vec<TokenisedFileData>,
    hash: HashMap<String, TokenisedFileData>,

    pos: i64,
    current_line_no: i64,
    current_token: Token,
    current_line: Vec<Token>,
    current_file: TokenisedFileData,
    current_object: String,
}

impl Parser {
    pub fn new(files: Vec<TokenisedFileData>) -> Parser {
        let mut hash = HashMap::new();
        for file in &files {
            hash.insert(file.filepath.clone(), file.clone());
        }
        Parser {
            files: files.clone(),
            hash,

            pos: 0,
            current_line_no: 0,
            current_token: Token::Other('a'),
            current_line: Vec::new(),
            current_file: files[0].clone(),
            current_object: String::new(),
        }
    }

    fn peek(&mut self) -> Token {
        let next_pos = self.pos + 1;
        let mut next_token = Token::None;
        if self.pos < self.current_line.len().try_into().unwrap() {
            next_token = self.current_line[next_pos as usize].clone();
            next_token
        } else {
            let line = self.peek_line();
            if line.is_empty() {
                next_token = Token::Eof;
                Token::Eof
            } else {
                next_token = line[0].clone();
                next_token
            }
        }

    }


    fn advance(&mut self) -> Token {
        self.pos += 1;
        if self.pos < self.current_line.len().try_into().unwrap() {
            self.current_token = self.current_line[self.pos as usize].clone();
            self.current_token.clone()
        } else {
            // println!("CHANGING LINE FROM {:?}", self.current_line);
            let line = self.advance_line();
            if line.is_empty() {
                self.current_token = Token::Eof;
                Token::Eof
            } else {
                self.current_token = line[0].clone();
                self.current_token.clone()
            }
        }
    }

    fn advance_to_eol(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let len = self.current_line.len() as i64;

        while self.pos < len {
            tokens.push(self.current_line[self.pos as usize].clone());
            self.pos += 1;
        }

        tokens
    }

    fn advance_up_to(&mut self, upto: Token) -> Vec<Token> {
        let mut tokens = Vec::new(); // : -> <T>
        self.advance();
        while self.current_token != upto {
            tokens.push(self.current_token.clone());
            self.advance();
        }
        self.advance(); // skip upto token

        return tokens

    }

    fn expect(&mut self, expected: Token) -> Token {
        self.advance();

        if self.current_token != expected {
            panic!("Expected {:?}, but found {:?}", expected, self.current_token)
        }

        self.current_token.clone()

    }

    fn advance_line(&mut self) -> Vec<Token> {
        while (self.current_line_no as usize) < self.current_file.lines.len() {
            println!("CHANGING LINE FROM {:?}", self.current_line);
            self.pos = 0;
            self.current_line_no += 1;
            self.current_line = self.current_file.lines[&self.current_line_no].to_vec();
            // println!("CHANGING LINE ---- NEW LINE : {:?}", self.current_line);
            if !self.current_line.is_empty() {
                self.current_token = self.current_line[0].clone();
                return self.current_line.clone();
            }
        }
        Vec::new()
    }

    fn peek_line(&self) -> Vec<Token> {
        let mut line_no = self.current_line_no;
        while (line_no as usize) < self.current_file.lines.len() {
            let line = self.current_file.lines[&line_no].to_vec();
            line_no += 1;
            if !line.is_empty() {
                return line;
            }
        }
        Vec::new()
    }

    pub fn parse(&mut self) {
        self.display_file_as_tokens();
        println!("\n");

        let possible_objects = self.possible_objects();

        let mut nodes: Vec<ASTNode> = Vec::new();

        for possible_object in possible_objects {
            let filepath = possible_object.0;

            let line_no = possible_object.1;
            self.current_line_no = line_no;

            let file = self.hash.get(&filepath).unwrap();
            self.current_file = file.clone();

            let lines = file.lines.clone();

            let line = lines.get(&line_no);
            self.current_line = line.unwrap().clone();

            self.pos = 0;
            self.current_token = self.current_line[self.pos as usize].clone();

            while self.current_token != Token::BlockClose && self.current_token != Token::Eof {
                // println!("RUNNING PARSE TOKEN FROM WHILE LOOP IN PARSE");
                let node = self.parse_token();
                if node == ASTNode::Eof {
                    break;
                }
                nodes.push(node);
            }

        }

        println!("\n \n");
        for n in nodes.clone() {
            println!("NODE : {:?}", n);
        }
        println!("\n");
        println!("{:?}", nodes.clone());
    }

    pub fn display_file_as_tokens(&self) {
        // println!("Displaying file: {}\n", self.current_file.filepath);

        let mut items: Vec<(&i64, &Vec<Token>)> =
            self.current_file.lines.iter().collect();

        items.sort_by_key(|&(line_no, _)| *line_no);

        for (line_no, tokens) in items {
            print!("Line {} | ", line_no);

            for token in tokens {
                print!("{:?} ", token);
            }

            println!();
        }
    }

    pub fn possible_objects(&mut self) -> Vec<(String, i64)> {
        let mut line_numbers_of_possible_objects: Vec<(String, i64)> = Vec::new();
        for file in &self.files {
            for lines in &file.lines {
                if lines.1.contains(&Token::ObjectDeclaration) {
                    line_numbers_of_possible_objects.push((file.filepath.clone(), *lines.0));
                    // println!("{} {:?} {:?}", lines.0, lines.1, file.filepath)
                }
            }
        }

        return line_numbers_of_possible_objects
    }

    // class Parent:
    //     def __init__(self) -> None:
    //         self.parent = None

    fn parse_token(&mut self) -> ASTNode {
        // println!("RUNNING PARSE TOKEN {:?} {:?}", self.current_token, self.current_line);
        match self.current_token {
            Token::ObjectDeclaration => self.handle_object(false),
            Token::Publicity(public) => self.handle_public_object(public),

            Token::VariableDeclaration | Token::SelfToken => self.handle_variable_declaration(),
            Token::FunctionDeclaration => self.handle_function_declaration(),

            Token::Indent(_) => {
                self.advance();
                self.parse_token()
            },

            Token::Eof => ASTNode::Eof,

            // _ => panic!("Cant parse following token : {:?}, at line {}: {:?}", self.current_token, self.current_line_no, self.current_line)
            _ => {
                println!("Skipping unrecognised token: {:?}", self.current_token);
                self.advance();
                ASTNode::None
            }
        }
    }


    fn parse_type(&mut self) -> String {
        let mut tokens = Vec::new();
        let mut count = 0;
        let mut first_loop = true;

        loop {
            match self.current_token {
                Token::Parenthesis('(') => count += 1,
                Token::Parenthesis(')') => count -= 1,
                Token::Bracket('[') => count += 1,
                Token::Bracket(']') => count -= 1,
                Token::BlockOpen('{') => count += 1,
                Token::BlockClose => count -= 1,
                Token::AngleBracket('<') => count += 1,
                Token::AngleBracket('>') => count -= 1,


                Token::Comma | Token::Colon if count == 0 && first_loop == false => break,


                Token::Eof => panic!("eof flag?"),

                _ => {},
            };

            first_loop = false;
            tokens.push(self.current_token.clone());
            self.advance();
        };



        let mut stokens = Vec::new();
        for token in tokens {
            stokens.push(token.to_string());
        }

        stokens.join("")

    }


    fn get_identifier(&mut self) -> String {
        match self.current_token.clone() {
            Token::Identifier(id) => id,
            Token::SelfToken => {
                self.advance();
                self.advance();
                match self.current_token.clone() {
                    Token::Identifier(id) => id,
                    _ => panic!()
                }
            }
            _ => panic!()
        }
    }

    fn parse_parenthesis(&mut self, parenth_open: Token, parenth_close: Token) -> Vec<Parameter> {
        let mut parameters = Vec::new();
        if self.current_token == parenth_open {
            self.advance(); // skip (

            while self.current_token != parenth_close {
                match self.current_token {

                    // Cover identfiers and self.
                    Token::Identifier(_) | Token::SelfToken => {
                        // get identfier
                        let identifier = match self.current_token.clone() {
                            Token::Identifier(id) => id,
                            Token::SelfToken => {


                                let next = self.peek();


                                if next == Token::Period || next == Token::Connect {
                                    self.advance(); // skip . | ::
                                    self.advance();
                                    match self.current_token.clone() {
                                        Token::Identifier(id) => id,
                                        _ => panic!("Unexpected token {:?}", self.current_token)
                                    }
                                } else {
                                    String::from("Self")
                                }
                            }
                            _ => panic!()
                        };

                        self.advance(); // move past identifier ( identifier -> ? ...  | possibly3

                        let mut ptype = None;
                        if self.current_token != Token::Parenthesis(')') {
                            ptype = Some(self.parse_type());
                        }

                        let param = Parameter {
                            identifier,
                            ptype: ptype,
                        };

                        parameters.push(param);

                        if self.current_token == Token::Comma {
                            self.advance();
                        }
                    },


                    // Cover all types and other tokens eg ([{
                    Token::Type(_) | Token::Other(_) => {
                        let ptype = self.parse_type();
                        self.advance();
                        let identifier = self.get_identifier();

                        let param = Parameter {
                            identifier,
                            ptype: Some(ptype)
                        };

                        parameters.push(param)
                    }

                    _ => panic!("Unexpected token {:?} {:?}", self.current_token, self.current_line)
                }

            }

            self.advance(); // skip )
        }

        parameters

    }

    fn handle_object(&mut self, public: bool) -> ASTNode {
        self.advance(); // skip ObjectDeclaration

        let id = self.current_token.clone(); // ObjectDeclaration -> Identifier
                                             //
        let identifier = match id {
            Token::Identifier(id) => id,
            _ => panic!("Expected identifier")
        };

        self.current_object = identifier.clone();
        self.advance();

        let parents = self.parse_parenthesis(Token::Parenthesis('('), Token::Parenthesis(')'));

        let should_be_block_open = self.current_token.clone();

        let mut block = Vec::new();
        if should_be_block_open == Token::BlockOpen('{') && should_be_block_open == Token::Colon {
            block = self.handle_block();
            // println!("block {:?}", block);
        } else {
            panic!("Panicked at line {:?}", self.current_line);
        }

        let object = Object {
            identifier,
            block,
            variables: HashMap::new(),
            public,
            parent: None,
       };

        ASTNode::Object(object)
    }

    fn handle_block(&mut self) -> Vec<ASTNode> {
        match self.current_token {
            Token::BlockOpen(_) => {
                self.advance(); // skip block open
                let mut block = Vec::new();

                while self.current_token != Token::BlockClose || self.current_token != Token::Eof {
                    block.push(self.parse_token());
                    self.advance();
                }
                self.advance(); // skip }

                block
            },

            Token::Colon => {
                self.advance(); // skip block open :
                let mut block = Vec::new();

                while self.current_line.is_empty() {
                    self.advance_line();
                }

                let starting_indent = match self.current_line[0].clone() {
                    Token::Indent(i) => i,
                    _ => panic!("Expected indent {:?} {:?}", self.current_token, self.current_line)
                };


                loop {
                    if self.current_line.is_empty() {
                        self.advance_line();
                        continue
                    }

                    let current_indent = match self.current_line[0].clone() {
                        Token::Indent(i) => i,
                        _ => break
                    };

                    if current_indent < starting_indent || self.current_token == Token::Eof {
                        break
                    }

                    let node = self.parse_token();
                    if node != ASTNode::None {
                        block.push(node);
                    }
                    self.advance_line();
                }

                block

            },

            _ => Vec::new(),
        }
    }

    fn handle_public_object(&mut self, public: bool) -> ASTNode {
        self.advance(); // skip pub
        self.handle_object(public)
    }


    fn handle_variable_declaration(&mut self) -> ASTNode {
       match self.current_file.file_type.as_str() {
           "python" => {
                self.expect(Token::Period); // self -> .
                let identifier = match self.advance() {
                    Token::Identifier(id) => id,
                    _ => panic!("Expected identifier"),
                };
                self.advance(); // move to : or =

                if self.current_token == Token::Equals { // self -> . -> identifier -> = -> value
                    let value = self.advance_to_eol(); // move onto value

                    return ASTNode::Variable(Variable {
                        identifier,
                        value: Some(value),
                        vtype: None,
                        parent: self.current_object.clone(),
                    })
                } else if self.current_token == Token::Colon {
                    let vtype = self.advance_up_to(Token::Equals);
                    let value = self.advance_to_eol();

                    return ASTNode::Variable(Variable {
                        identifier,
                        value: Some(value),
                        vtype: Some(vtype),
                        parent: self.current_object.clone(),
                    })

                } else {
                    panic!("Error with python variable ObjectDeclaration")
                }
           },

           "rust" => {
                let identifier = match &self.current_token {
                    Token::Identifier(id) => id.to_string(),
                    _ => panic!("Expected identifier, not {:?}", self.current_token)
                };
                self.advance(); // Identifier -> :

                let vtype = self.advance_to_eol();

                let parent = self.current_object.clone();
                let variable = Variable {
                    identifier,
                    value: None,
                    vtype: Some(vtype),
                    parent

                };

                ASTNode::Variable(variable)
           }

           _ => panic!("File type not currently supported")
       }

    }

    fn handle_function_declaration(&mut self) -> ASTNode {
        let identifier = match self.advance() {
            Token::Identifier(id) => id,
            _ => panic!("Expected identifier")
        }; // skip def| fn | etc  -> Identifier


        self.expect(Token::Parenthesis('('));
        let parameters = self.parse_parenthesis(Token::Parenthesis('('), Token::Parenthesis(')'));

        let mut ftype = Vec::new();
        match &self.current_token {
            Token::Arrow => { // class Parent(child) -> Type:
                self.advance(); // skip ->
                ftype = self.advance_to_eol();
                ftype.pop(); // remove block open : | {
                self.handle_block();
            },

            Token::Identifier(id) => { //


            },

            Token::Other(o) => {

            },

            Token::BlockOpen('{') | Token::Colon => {
                self.handle_block();
            },

            _ => panic!("Unexpected token in function declaration {:?}", self.current_token)
        }

        let mut ftype_strings = Vec::new();
        for token in ftype {
            ftype_strings.push(token.to_string());
        }

        let return_type: String = ftype_strings.join("");

        let function = Function {
            identifier,
            parameters,
            return_type,
        };

        ASTNode::Function(function)

    }
}
