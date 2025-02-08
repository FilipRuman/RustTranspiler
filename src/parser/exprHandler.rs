use crate::{
    ast::Expression,
    lookup::BindingPower,
    parser::{self, Parser},
    tokens::Token,
};

pub fn handle_number(parser: &mut Parser) -> Expression {
    let number = parser.advance().value.parse::<f32>().unwrap();
    return Expression::Number(number);
}
pub fn handle_string(parser: &mut Parser) -> Expression {
    return Expression::String(parser.advance().value.to_string());
}
pub fn handle_identifier(parser: &mut Parser) -> Expression {
    return Expression::Symbol(parser.advance().value.to_string());
}
pub fn parse_binary_expression(
    parser: &mut Parser,
    left: Expression,
    bp: BindingPower,
) -> Expression {
    let reference = parser.advance();
    let operator_token: Token = Token {
        kind: reference.kind,
        value: reference.value.clone(),
        line: reference.line,
    };
    let right = parse_expr(parser, bp);
    return Expression::Binary(Box::new(left), operator_token, Box::new(right));
}
pub fn parse_expr(parser: &mut Parser, bp: BindingPower) -> Expression {
    let mut token = parser.current_token();
    let option = parser.lookups.nud_lu.get(&token.kind);
    if option == None {
        panic!(
            "Nud handler expected for token kind: {:?} token index: {}",
            { token.kind },
            parser.pos
        );
    }
    let nud = option.unwrap();
    let mut left = nud(parser);
    // println!("token kind {:?}", parser.current_token().kind);

    while parser.lookups.current_binding_power(parser) as u8 > bp as u8 {
        token = parser.current_token();

        let ledOption = parser.lookups.left_denoted_lu.get(&token.kind);
        if ledOption == None {
            panic!("Led handler expected for token kind: {:?}", { token.kind });
        }
        left = ledOption.unwrap()(
            parser,
            left,
            parser.lookups.binding_power_lu[&parser.current_token().kind],
        );
    }
    return left;
}

pub fn parse_assignment(parser: &mut Parser, left: Expression, bp: BindingPower) -> Expression {
    let operator = parser.advance().clone();
    let rhs = parse_expr(parser, bp);

    return Expression::Assignment(Box::new(left), operator, Box::new(rhs));
}

pub fn parse_prefix(parser: &mut Parser) -> Expression {
    let operator_token = parser.advance().clone();
    let rhs = parse_expr(parser, BindingPower::Default);

    return Expression::Prefix(operator_token, Box::new(rhs));
}

pub fn parse_grouping_expr(parser: &mut Parser) -> Expression {
    parser.advance();

    let expression = parse_expr(parser, BindingPower::Default);
    parser.expect(crate::tokens::TokenKind::CloseParen);
    return expression;
}
