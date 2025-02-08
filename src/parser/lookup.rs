use core::num;
use std::{cmp, collections::HashMap};

use crate::{
    ast::{Expression, Statement},
    exprHandler,
    parser::Parser,
    stmtHandler,
    tokens::{Token, TokenKind},
};
#[derive(Copy, Clone)]
pub enum BindingPower {
    Default,
    Comma,
    Assignment,
    Logical,
    Relational,
    Additive,
    Multiplicative,
    Unary,
    Call,
    Member,
    Primary,
}
pub struct Lookups {
    pub binding_power_lu: HashMap<TokenKind, BindingPower>,
    pub nud_lu: HashMap<TokenKind, fn(&mut Parser) -> Expression>,
    pub left_denoted_lu:
        HashMap<TokenKind, fn(&mut Parser, Expression, BindingPower) -> Expression>,
    pub statement_lu: HashMap<TokenKind, fn(&mut Parser) -> Statement>,
}
impl Lookups {
    pub fn new() -> Lookups {
        let mut lookUp = Lookups {
            binding_power_lu: HashMap::new(),
            nud_lu: HashMap::new(),
            left_denoted_lu: HashMap::new(),
            statement_lu: HashMap::new(),
        };
        lookUp.create_token_lookups();
        return lookUp;
    }
    fn led(
        &mut self,
        kind: TokenKind,
        bp: BindingPower,
        led_fn: fn(&mut Parser, Expression, BindingPower) -> Expression,
    ) {
        self.binding_power_lu.insert(kind, bp);
        self.left_denoted_lu.insert(kind, led_fn);
    }
    fn nud(&mut self, kind: TokenKind, nud_fn: fn(&mut Parser) -> Expression) {
        self.nud_lu.insert(kind, nud_fn);
    }
    fn stmt(&mut self, kind: TokenKind, stmt_fn: fn(&mut Parser) -> Statement) {
        self.binding_power_lu.insert(kind, BindingPower::Default);
        self.statement_lu.insert(kind, stmt_fn);
    }
    pub fn current_binding_power(&self, parser: &Parser) -> BindingPower {
        let option = self.binding_power_lu.get(&parser.current_token().kind);
        match option {
            Some(bp) => return bp.to_owned(),
            None => return BindingPower::Default,
        }
    }
    fn create_token_lookups(&mut self) {
        self.led(
            TokenKind::Assignment,
            BindingPower::Assignment,
            exprHandler::parse_assignment,
        );
        self.led(
            TokenKind::PlusEquals,
            BindingPower::Assignment,
            exprHandler::parse_assignment,
        );

        self.led(
            TokenKind::MinusEquals,
            BindingPower::Assignment,
            exprHandler::parse_assignment,
        );
        // TODo /= *=

        //
        self.led(
            TokenKind::And,
            BindingPower::Logical,
            exprHandler::parse_binary_expression,
        );
        self.led(
            TokenKind::Or,
            BindingPower::Logical,
            exprHandler::parse_binary_expression,
        );
        self.led(
            TokenKind::DotDot,
            BindingPower::Logical,
            exprHandler::parse_binary_expression,
        );

        self.led(
            TokenKind::Less,
            BindingPower::Relational,
            exprHandler::parse_binary_expression,
        );
        self.led(
            TokenKind::LessEquals,
            BindingPower::Relational,
            exprHandler::parse_binary_expression,
        );
        self.led(
            TokenKind::Greater,
            BindingPower::Relational,
            exprHandler::parse_binary_expression,
        );
        self.led(
            TokenKind::GreaterEquals,
            BindingPower::Relational,
            exprHandler::parse_binary_expression,
        );
        self.led(
            TokenKind::Equals,
            BindingPower::Relational,
            exprHandler::parse_binary_expression,
        );
        self.led(
            TokenKind::NotEquals,
            BindingPower::Relational,
            exprHandler::parse_binary_expression,
        );

        self.led(
            TokenKind::Plus,
            BindingPower::Additive,
            exprHandler::parse_binary_expression,
        );
        self.led(
            TokenKind::Minus,
            BindingPower::Additive,
            exprHandler::parse_binary_expression,
        );
        self.led(
            TokenKind::Star,
            BindingPower::Multiplicative,
            exprHandler::parse_binary_expression,
        );
        self.led(
            TokenKind::Slash,
            BindingPower::Multiplicative,
            exprHandler::parse_binary_expression,
        );
        self.led(
            TokenKind::Percent,
            BindingPower::Multiplicative,
            exprHandler::parse_binary_expression,
        );

        self.led(
            TokenKind::PlusPlus,
            BindingPower::Multiplicative,
            exprHandler::parse_binary_expression,
        );
        self.led(
            TokenKind::MinusMinus,
            BindingPower::Multiplicative,
            exprHandler::parse_binary_expression,
        );

        self.nud(TokenKind::Number, exprHandler::handle_number);
        self.nud(TokenKind::String, exprHandler::handle_string);
        self.nud(TokenKind::Identifier, exprHandler::handle_identifier);
        self.nud(TokenKind::Minus, exprHandler::parse_prefix);

        self.nud(TokenKind::OpenParen, exprHandler::parse_grouping_expr);

        self.stmt(
            TokenKind::I32,
            stmtHandler::parse_variable_declaration_statement,
        );
        self.stmt(
            TokenKind::I64,
            stmtHandler::parse_variable_declaration_statement,
        );
        self.stmt(
            TokenKind::F32,
            stmtHandler::parse_variable_declaration_statement,
        );
        self.stmt(
            TokenKind::Str,
            stmtHandler::parse_variable_declaration_statement,
        );
        self.stmt(
            TokenKind::Var,
            stmtHandler::parse_variable_declaration_statement,
        );
    }
}
