use std::collections::HashMap;

use crate::{
    expression::{
        self, parse_assignment, parse_binary_expr, parse_expr, parse_grouping,
        parse_identifier_nod, parse_keyword_nod, parse_number_nod, parse_prefix_nod,
        parse_string_nod, parse_variable_declaration, Expression,
    },
    parser::{self, Parser},
    tokens::TokenKind,
};
pub struct Lookup {
    pub binding_power_lu: HashMap<TokenKind, i8>,

    pub led_lu: HashMap<TokenKind, fn(&mut Parser, &i8, Expression) -> Expression>,
    pub nod_lu: HashMap<TokenKind, fn(&mut Parser) -> Expression>,
}
impl Lookup {
    pub fn get_led(
        &self,
        token_kind: TokenKind,
    ) -> &fn(&mut Parser, &i8, Expression) -> Expression {
        self.led_lu.get(&token_kind).expect(&format!(
            "led was not found for token kind: {:?}",
            token_kind
        ))
    }
    pub fn get_nod(&self, token_kind: TokenKind) -> &fn(&mut Parser) -> Expression {
        self.nod_lu.get(&token_kind).expect(&format!(
            "nod was not found for token kind: {:?}",
            token_kind
        ))
    }
    pub fn get_bp(&self, token_kind: TokenKind) -> &i8 {
        self.binding_power_lu.get(&token_kind).expect(&format!(
            "bp was not found for token kind: {:?}",
            token_kind
        ))
    }
    fn led(
        &mut self,
        token_kind: TokenKind,
        bp: i8,
        function: fn(&mut Parser, &i8, Expression) -> Expression,
    ) {
        self.led_lu.insert(token_kind, function);
        self.binding_power_lu.insert(token_kind, bp);
    }

    fn nod(&mut self, token_kind: TokenKind, bp: i8, function: fn(&mut Parser) -> Expression) {
        self.nod_lu.insert(token_kind, function);
        self.binding_power_lu.insert(token_kind, bp);
    }

    pub fn new() -> Lookup {
        let mut lookup = Lookup {
            binding_power_lu: HashMap::new(),
            led_lu: HashMap::new(),
            nod_lu: HashMap::new(),
        };

        lookup.led(TokenKind::Assignment, 1, parse_assignment);
        lookup.led(TokenKind::Equals, 1, parse_assignment);
        lookup.led(TokenKind::PlusEquals, 1, parse_assignment);
        lookup.led(TokenKind::MinusEquals, 1, parse_assignment);
        // lookup.led(TokenKind::StarEquals, 1, parse_assignment);
        // lookup.led(TokenKind::SlashEquals, 1, parse_assignment);

        lookup.led(TokenKind::Plus, 2, parse_binary_expr);
        lookup.led(TokenKind::Minus, 2, parse_binary_expr);
        lookup.led(TokenKind::Star, 3, parse_binary_expr);
        lookup.led(TokenKind::Slash, 3, parse_binary_expr);

        lookup.led(TokenKind::Less, 1, parse_binary_expr);
        lookup.led(TokenKind::LessEquals, 1, parse_binary_expr);
        lookup.led(TokenKind::Greater, 1, parse_binary_expr);
        lookup.led(TokenKind::GreaterEquals, 1, parse_binary_expr);
        lookup.led(TokenKind::Equals, 1, parse_binary_expr);

        lookup.nod(TokenKind::OpenParen, 4, parse_grouping);
        lookup.nod(TokenKind::CloseParen, 0, parse_grouping);

        lookup.nod(TokenKind::Str, 0, parse_variable_declaration);
        lookup.nod(TokenKind::I16, 0, parse_variable_declaration);
        lookup.nod(TokenKind::I32, 0, parse_variable_declaration);
        lookup.nod(TokenKind::U16, 0, parse_variable_declaration);
        lookup.nod(TokenKind::U32, 0, parse_variable_declaration);
        lookup.nod(TokenKind::Bool, 0, parse_variable_declaration);

        lookup.nod(TokenKind::String, 0, parse_string_nod);
        lookup.nod(TokenKind::Identifier, 0, parse_identifier_nod);
        lookup.nod(TokenKind::Number, 0, parse_number_nod);

        lookup.nod(TokenKind::Minus, 0, parse_prefix_nod);
        lookup.nod(TokenKind::Plus, 0, parse_prefix_nod);

        lookup.nod(TokenKind::SemiColon, -1, parse_keyword_nod);
        lookup.binding_power_lu.insert(TokenKind::EndOfFile, -1);

        lookup
    }
}
