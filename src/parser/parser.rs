use crate::{
    ast::Statement,
    lexer, lookup, stmtHandler,
    tokens::{Token, TokenKind},
};
struct Error {}
pub struct Parser {
    pub lookups: lookup::Lookups,
    pub tokens: Vec<Token>,
    pub pos: u16,
    pub errors: Vec<Error>,
}
impl Parser {
    pub fn current_token(&self) -> &Token {
        return &self.tokens[self.pos as usize];
    }
    pub fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.pos as usize];
        self.pos += 1;
        return token;
    }
    pub fn next(&self) -> &Token {
        let token = &self.tokens[(self.pos + 1) as usize];
        return token;
    }
    /// is at the end of file
    pub fn eof(&self) -> bool {
        return self.pos as usize >= self.tokens.len()
            || self.current_token().kind == TokenKind::Eof;
    }

    pub fn expectError(&mut self, expectedKind: TokenKind, error: Option<&str>) -> &Token {
        let token = self.current_token();

        if token.kind != expectedKind {
            match error {
                Some(text) => panic!("{}", text),
                None => println!("expected {:?} but received {:?}", expectedKind, token.kind),
            }
        }

        return self.advance();
    }

    pub fn expect(&mut self, expectedKind: TokenKind) -> &Token {
        return self.expectError(expectedKind, None);
    }
}
/// returns: Statement::Block
pub fn parse(tokens: Vec<Token>) -> Statement {
    let mut statements: Vec<Statement> = Vec::new(); //statements
    let lookups = lookup::Lookups::new();
    let mut parser = Parser {
        tokens,
        pos: 0,
        errors: Vec::new(),
        lookups,
    };

    while !parser.eof() {
        statements.push(stmtHandler::parse_statement(&mut parser));
    }

    return Statement::Block(statements);
}
