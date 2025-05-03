use crate::{
    expression::{self, parse_expr, Expression},
    lookup::Lookup,
    tokens::{Token, TokenKind},
};

pub struct Parser {
    pub index: usize,
    pub tokens: Vec<Token>,
    pub lookup: Lookup,
}

const TAKEN_ARRAY_LENGTH_CHECK_SAFETY_CHECK: bool = false;
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            index: 0,
            tokens,
            lookup: Lookup::new(),
        }
    }
    pub fn get_token(&self, index: usize) -> &Token {
        if TAKEN_ARRAY_LENGTH_CHECK_SAFETY_CHECK && index >= self.tokens.len() {
            panic!(
                "index:{} was greater than tokens array length:{} !",
                index,
                self.tokens.len()
            );
        }

        &self.tokens[index]
    }
    pub fn advance(&mut self) -> &Token {
        self.index += 1;
        &self.get_token(self.index - 1)
    }
    pub fn current_token(&self) -> &Token {
        &self.get_token(self.index)
    }
    pub fn current_token_kind(&self) -> &TokenKind {
        &self.get_token(self.index).kind
    }
    pub fn current_bp(&self) -> &i8 {
        self.lookup.get_bp(self.get_token(self.index).kind)
    }

    pub fn expect(&self, expected: &TokenKind) -> &Token {
        let current = self.current_token();
        if &current.kind == expected {
            return current;
        }

        panic!("Expected: {:?} but found: {:?} ", expected, current);
    }
}
pub fn parse(tokens: Vec<Token>) -> Vec<Expression> {
    let mut parser = Parser::new(tokens);

    let mut parsed_lines: Vec<Expression> = Vec::new();
    while parser.current_token_kind() != &TokenKind::EndOfFile {
        println!("\n parse new line :: \n");
        parsed_lines.push(parse_expr(&mut parser, &0));
    }

    parsed_lines
}
