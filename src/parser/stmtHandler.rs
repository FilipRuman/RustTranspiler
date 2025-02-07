use std::fmt::Error;

use crate::{
    ast::{Expression, Statement},
    exprHandler,
    lexer::{self, Lexer},
    parser::{self, Parser},
    tokens::TokenKind,
};

pub fn parse_statement(parser: &mut Parser) -> Statement {
    let token = parser.current_token();

    let option = parser.lookups.statement_lu.get(&token.kind);

    match option {
        Some(statement_function) => {
            return statement_function(parser);
        }
        None => {
            let expression = exprHandler::parse_expr(parser, crate::lookup::BindingPower::Default);
            parser.expect(TokenKind::SemiColon);

            return Statement::Expression(expression);
        }
    }
}

pub fn parse_variable_declaration_statement(parser: &mut Parser) -> Statement {
    let varType = parser.advance().kind;
    let mutable = parser.next().kind == TokenKind::Mut;

    let varName = parser
        .expectError(
            TokenKind::Identifier,
            Some(&format!(
                "variable name not found at token index: {}",
                parser.pos
            )),
        )
        .value
        .to_string();
    parser.expect(TokenKind::Assignment);
    let assignedValue = exprHandler::parse_expr(parser, crate::lookup::BindingPower::Assignment);

    parser.expect(TokenKind::SemiColon);
    return Statement::VariableDeclaration(varName.to_string(), mutable, assignedValue, varType);
}
