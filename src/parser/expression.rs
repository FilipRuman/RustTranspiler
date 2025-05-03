use std::env::{consts, VarError};

use crate::{
    parser::{self, Parser},
    tokens::{Token, TokenKind},
};

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f32),
    String(String),
    Identifier(String),
    Prefix(Token, Box<Expression>),
    Keyword(TokenKind),
    // target operator value
    Assignment(Box<Expression>, Token, Box<Expression>),
    // type name mutable
    VariableDeclaration(TokenKind, String, bool),
    //TODO:
    Grouping(Box<Expression>),

    Binary(Box<Expression>, Token, Box<Expression>),
}

pub fn parse_expr(parser: &mut Parser, bp: &i8) -> Expression {
    let nod = parser.current_token();

    debug_expression(&format!("parse_expr: nod:{:?} bp:{}", nod, bp));
    let mut left = parser.lookup.get_nod(nod.kind)(parser);

    while parser.current_bp() > bp {
        let led = parser.current_token().kind.clone();
        let led_fn = parser.lookup.get_led(led);

        debug_expression(&format!("expr led call ->>:",));

        left = led_fn(parser, &parser.current_bp().to_owned(), left);

        debug_expression(&format!(
            "expr while loop: current kind{:?} current bp:{} bp:{}",
            parser.current_token_kind(),
            parser.current_bp(),
            bp
        ));
    }
    return left;
}
pub fn parse_variable_declaration(parser: &mut Parser) -> Expression {
    let variable_type = parser.advance().kind;

    let mutable = parser.current_token_kind() == &TokenKind::Mut;
    if mutable {
        parser.advance();
    }

    let name = (&parser.expect(&TokenKind::Identifier).value).to_owned();

    parser.advance();
    return Expression::VariableDeclaration(variable_type, name, mutable);
}
pub fn parse_assignment(parser: &mut Parser, _: &i8, target: Expression) -> Expression {
    debug_expression(&format!(
        "assignment_expr: target: {:?} current kind: {:?} ",
        target,
        parser.current_token_kind(),
    ));

    let operator = parser.current_token().clone();
    parser.advance();
    let value = parse_expr(parser, &0);
    return Expression::Assignment(Box::new(target), operator, Box::new(value));
}

pub fn parse_binary_expr(parser: &mut Parser, bp: &i8, left: Expression) -> Expression {
    let operator_original = parser.advance();
    let operator = operator_original.clone();

    let right = parse_expr(parser, &bp);

    Expression::Binary(Box::new(left), operator, Box::new(right))
}
pub fn parse_grouping(parser: &mut Parser) -> Expression {
    parser.advance();

    let expression_inside = parse_expr(parser, &0);
    parser.expect(&TokenKind::CloseParen);

    debug_expression(&format!("parsed grouping ",));

    Expression::Grouping(Box::new(expression_inside))
}
pub fn parse_number_nod(parser: &mut Parser) -> Expression {
    debug_expression(&format!(
        "parse number nod {:?}",
        parser.current_token().value
    ));
    Expression::Number(parser.advance().value.parse::<f32>().unwrap())
}
pub fn parse_prefix_nod(parser: &mut Parser) -> Expression {
    debug_expression(&format!(
        "parse prefix nod {:?}",
        parser.current_token().value
    ));

    let operator = parser.advance().to_owned();
    let target = Box::new(parse_expr(parser, &0));

    Expression::Prefix(operator, target)
}

pub fn parse_identifier_nod(parser: &mut Parser) -> Expression {
    debug_expression(&format!(
        "parse identifier nod {:?}",
        parser.current_token().value
    ));
    Expression::Identifier(parser.advance().value.to_string())
}
pub fn parse_keyword_nod(parser: &mut Parser) -> Expression {
    debug_expression(&format!(
        "parse keyword nod {:?}",
        parser.current_token().value
    ));

    Expression::Keyword(parser.advance().kind)
}
pub fn parse_string_nod(parser: &mut Parser) -> Expression {
    debug_expression(&format!(
        "parse string  nod {:?}",
        parser.current_token().value
    ));
    Expression::String(parser.advance().value.to_string())
}

const SHOW_EXPRESSION_DEBUG: bool = true;
fn debug_expression(text: &str) {
    if !SHOW_EXPRESSION_DEBUG {
        return;
    }

    println!("{}", text);
}
