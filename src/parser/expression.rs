use std::env::{consts, VarError};

use crate::{
    lexer,
    parser::{self, Parser},
    tokens::{Token, TokenKind},
    types::{parse_symbol_type, parse_type, Type},
};

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f32),
    String(String),
    Identifier(String),
    Prefix {
        prefix: Token,
        value: Box<Expression>,
    },
    Keyword(TokenKind),
    // target operator value
    Assignment {
        target: Box<Expression>,
        operator: Token,
        value: Box<Expression>,
    },
    // type name mutable
    VariableDeclaration {
        var_type: Type,
        name: String,
        mutable: bool,
    },
    //TODO:
    Grouping(Box<Expression>),
    Class {
        public: bool,
        name: String,
        properties: Vec<Expression>,
        functions: Vec<Expression>,
    },
    ClassProperty {
        var_name: String,
        var_type: Type,
    },
    ClassFunction {
        name: String, /* ,type : Type */
    },

    Binary {
        l: Box<Expression>,
        operator: Token,
        r: Box<Expression>,
    },
    ClassInstantiation {
        name: String,
        properties: Vec<Expression>,
    },
    ArrayInitialization {
        properties: Vec<Expression>,
    },
}

pub fn parse_expr(parser: &mut Parser, bp: &i8) -> Expression {
    debug_expression("      expr:");
    let nod = parser.current_token();
    let mut to_debug = format!("parse_expr: nod:{:?} bp:{} ", nod, bp,);
    let mut left = parser.lookup.get_nod(nod.kind)(parser);

    to_debug += &format!(
        "current_token_kind:{:?} current_bp:{}",
        parser.current_token_kind(),
        parser.lookup.get_bp(&TokenKind::Plus)
    );
    debug_expression(&to_debug);

    while parser.current_bp() > bp {
        let led = parser.current_token().kind.clone();
        let led_fn = parser.lookup.get_led(led);

        debug_expression(&format!("expr led call: led:{:?} ->>:", led));

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
pub fn parse_class(parser: &mut Parser) -> Expression {
    // class pub NAME {
    // i32 name ;
    // bool[] orher_name;
    // }
    parser.expect(&TokenKind::Class);
    let public = parser.current_token_kind() == &TokenKind::Pub;
    if public {
        parser.expect(&TokenKind::Pub);
    }
    let name = parser.expect(&TokenKind::Identifier).value.to_owned();
    debug_expression(&format!("parse class",));
    parser.expect(&TokenKind::OpenCurly);

    let mut properties = Vec::new();
    let mut functions = Vec::new();
    while parser.current_token_kind() != &TokenKind::EndOfFile
        && parser.current_token_kind() != &TokenKind::CloseCurly
    {
        debug_expression(&format!(
            "parse class property: {:?}",
            parser.current_token_kind()
        ));
        // Property
        if parser.current_token_kind() == &TokenKind::Identifier {
            let property_type = parse_type(parser, &0);
            let property_name = parser.expect(&TokenKind::Identifier).value.clone();
            parser.expect(&TokenKind::SemiColon);
            properties.push(Expression::ClassProperty {
                var_name: property_name,
                var_type: property_type,
            });
            continue;
        }
        // function
    }
    parser.expect(&TokenKind::CloseCurly);

    Expression::Class {
        public,
        name,
        functions,
        properties,
    }
}
pub fn parse_array_initialization(parser: &mut Parser) -> Expression {
    parser.expect(&TokenKind::OpenCurly);
    let mut properties = Vec::new();
    while parser.current_token_kind() != &TokenKind::EndOfFile
        && parser.current_token_kind() != &TokenKind::CloseCurly
    {
        properties.push(parse_expr(parser, &0));
    }

    parser.expect(&TokenKind::CloseCurly);

    Expression::ArrayInitialization { properties }
}
pub fn parse_class_instantiation(parser: &mut Parser, bp: &i8, left: Expression) -> Expression {
    let name = match left {
        Expression::Identifier(text) => text,
        _ => {
            panic!("left is not a identifier in parse_class_instantiation ")
        }
    };

    parser.expect(&TokenKind::OpenCurly);
    let mut properties = Vec::new();
    while parser.current_token_kind() != &TokenKind::EndOfFile
        && parser.current_token_kind() != &TokenKind::CloseCurly
    {
        properties.push(parse_expr(parser, &0));
    }

    parser.expect(&TokenKind::CloseCurly);
    Expression::ClassInstantiation {
        name,
        properties: properties,
    }
}
pub fn parse_variable_declaration(parser: &mut Parser) -> Expression {
    // let mut i32 name = 1+2;

    // move past let
    parser.advance();
    let mutable = parser.current_token_kind() == &TokenKind::Mut;
    if mutable {
        parser.advance();
    }
    let var_type = parse_type(parser, &0);

    let name = (&parser.expect(&TokenKind::Identifier).value).to_owned();

    debug_expression(&format!(
        "variable_declaration_expression: type{:?} mut:{} name:{} next_token_kind:{:?}",
        var_type,
        mutable,
        name,
        parser.current_token_kind(),
    ));
    return Expression::VariableDeclaration {
        var_type,
        name,
        mutable,
    };
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
    return Expression::Assignment {
        target: Box::new(target),
        operator,
        value: Box::new(value),
    };
}

pub fn parse_binary_expr(parser: &mut Parser, bp: &i8, left: Expression) -> Expression {
    let operator_original = parser.advance();
    let operator = operator_original.clone();
    debug_expression(&format!(
        "parsed binary: bp:{} operator{:?} right_token_kind{:?}",
        bp,
        operator,
        parser.current_token_kind()
    ));

    let right = parse_expr(parser, &bp);

    Expression::Binary {
        l: Box::new(left),
        operator,
        r: Box::new(right),
    }
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

    let prefix = parser.advance().to_owned();
    let value = Box::new(parse_expr(parser, &0));

    Expression::Prefix { prefix, value }
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
