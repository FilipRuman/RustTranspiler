use std::{default, path::StripPrefixError, process::Output};

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
        Expression::Binary { l, operator, r } => handle_binary_expr(*l, operator, *r),
        Expression::Assignment {
            target,
            operator,
            value,
        } => handle_assignment(*target, operator, *value),
        Expression::VariableDeclaration {
            var_type,
            name,
            mutable,
        } => handle_variable_declaration(var_type, name, mutable),
        Expression::Grouping(expression_inside) => handle_grouping(*expression_inside),
        Expression::Keyword(token_kind) => handle_keyword(token_kind),
        Expression::Prefix { prefix, value } => handle_prefix(prefix, *value),
        Expression::Class {
            public: _,
            name,
            properties,
            functions,
        } => handle_class(name, properties, functions),
        Expression::ClassProperty { var_name, var_type } => {
            handle_class_property(var_name, var_type)
        }
        Expression::ClassFunction { name } => todo!(),
        Expression::ClassInstantiation { name, properties } => {
            handle_class_instantiation(name, properties)
        }
        Expression::ArrayInitialization { properties } => handle_array_initialization(properties),
    }
}
fn handle_array_initialization(properties: Vec<Expression>) -> String {
    let mut properties_text = String::new();
    for property in properties {
        match property {
            Expression::Keyword(TokenKind::SemiColon) => continue,
            _ => {}
        }
        properties_text += &format!("{},", &handle_expr(property));
    }
    return format!("{{ {} }};\n", properties_text);
}

fn handle_class_instantiation(name: String, properties: Vec<Expression>) -> String {
    let mut properties_text = String::new();
    for property in properties {
        match property {
            Expression::Keyword(TokenKind::SemiColon) => continue,
            _ => {}
        }
        properties_text += &format!("{},\n", &handle_expr(property));
    }
    return format!("new {}{{\n{}}};\n", name, properties_text);
}
fn handle_class_property(var_name: String, var_type: Type) -> String {
    return format!("{} {}", handle_type(var_type), var_name);
}
fn handle_class(name: String, properties: Vec<Expression>, functions: Vec<Expression>) -> String {
    let mut properties_text = String::new();
    for property in properties {
        properties_text += &format!("public {};\n", &handle_expr(property));
    }
    let functions_text = String::new();
    return format!(
        "struct {} {{\n{}{}}};\n",
        name, properties_text, functions_text
    );
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
        TokenKind::Enum => todo!(),
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
    //     TokenKind::Str => "string",
    //     TokenKind::I32 => "long",
    //     TokenKind::I16 => "int",
    //     TokenKind::U32 => "long",
    //     TokenKind::U16 => "int",
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
        "str" => "string",
        "i32" => "long",
        "i16" => "int",
        "u32" => "ulong",
        "u16" => "uint",

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
