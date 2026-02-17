use crate::lexer::Token;

pub struct Parser {
   lines: Vec<(i64, Vec<Token>, String)>,
}

impl Parser {
    pub fn new(lines: Vec<(i64, Vec<Token>, String)>) -> Self {
        Parser {
            lines
        }
    }

    pub fn possible_objects(&mut self) {
        for line in &self.lines {
            let tkl = &line.1;
            let mut tkls: Vec<&Vec<Token>> = Vec::new();

            if tkl.contains(&Token::ObjectDeclaration) {
                tkls.push(tkl);
                println!("{:?}", tkl);
            }
        }
    }
}
