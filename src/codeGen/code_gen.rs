use std::{default, process::Output};

use crate::{
    expression::{self, Expression},
    tokens::{Token, TokenKind},
    type_lookup,
    types::Type,
};

pub fn convert_expressions_to_code(expressions: Vec<Expression>) -> String {
    let mut output: String = String::new();
    for expression in expressions {
        output += &handle_expr(expression);
    }

    output
}
fn handle_expr(expression: Expression) -> String {
    match expression {
        Expression::Number(value) => value.to_string(),
        Expression::String(value) => format!("\"{}\"", value),
        Expression::Identifier(value) => value,
        Expression::Binary(left, operator, right) => handle_binary_expr(*left, operator, *right),
        Expression::Assignment(target, operator, value) => {
            handle_assignment(*target, operator, *value)
        }
        Expression::VariableDeclaration(variable_type, variable_name, mutable) => {
            handle_variable_declaration(variable_type, variable_name, mutable)
        }
        Expression::Grouping(expression_inside) => handle_grouping(*expression_inside),
        Expression::Keyword(token_kind) => handle_keyword(token_kind),
        Expression::Prefix(prefix, target) => handle_prefix(prefix, *target),
    }
}
fn handle_prefix(prefix: Token, target: Expression) -> String {
    return format!("{}{}", prefix.value, handle_expr(target));
}
fn handle_grouping(expression_inside: Expression) -> String {
    return format!("{}", handle_expr(expression_inside));
}
fn handle_keyword(token_kind: TokenKind) -> String {
    match token_kind {
        TokenKind::SemiColon => ";\n".to_owned(),
        TokenKind::Fn => todo!(),
        TokenKind::Enum => todo!(),
        TokenKind::Class => todo!(),
        TokenKind::Public => todo!(),
        TokenKind::Mod => todo!(),
        TokenKind::As => todo!(),
        TokenKind::New => todo!(),
        TokenKind::Import => todo!(),
        TokenKind::For => todo!(),
        TokenKind::In => todo!(),
        TokenKind::If => todo!(),
        TokenKind::Else => todo!(),
        TokenKind::While => todo!(),
        TokenKind::Mut => todo!(),
        default => panic!("key word: {:?} doesn't have a handler", default),
    }
}
pub fn handle_variable_declaration(
    variable_type: Type,
    variable_name: String,
    mutable: bool,
) -> String {
    let type_str = handle_type(variable_type);
    // match variable_type {
    //     TokenKind::Str => "char[]",
    //     TokenKind::I32 => "long",
    //     TokenKind::I16 => "int",
    //     TokenKind::U32 => "unsigned long",
    //     TokenKind::U16 => "unsigned int",
    //     TokenKind::Bool => "bool",
    //
    //     default => panic!("variable type: {:?} doesn't have a handler", default),
    // };

    let mut_str = if mutable { "" } else { "const " };

    return format!("{}{} {}", mut_str, type_str, variable_name);
}
fn handle_type(var_type: Type) -> String {
    match var_type {
        Type::Symbol(symbol) => handle_symbol_type(symbol),
        Type::Array(type_inside) => format!("{}[]", handle_type(*type_inside)),
    }
}
fn handle_symbol_type(symbol: String) -> String {
    match symbol.as_str() {
        "str" => "char[]",
        "i32" => "long",
        "i16" => "int",
        "u32" => "unsigned long",
        "u16" => "unsigned int",

        default => default,
    }
    .to_string()
}

fn handle_assignment(target: Expression, operator: Token, value: Expression) -> String {
    let target_str = handle_expr(target);
    let operator_str = operator.value;
    let value_str = handle_expr(value);

    return format!("{} {} {}", target_str, operator_str, value_str);
}

fn handle_binary_expr(left: Expression, operator: Token, right: Expression) -> String {
    let left_str = handle_expr(left);
    let operator_str = operator.value;
    let right_str = handle_expr(right);

    return format!("({} {} {})", left_str, operator_str, right_str);
}
