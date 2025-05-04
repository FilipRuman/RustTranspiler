use std::collections::HashMap;

use crate::{
    parser::Parser,
    tokens::TokenKind,
    types::{parse_array_type, parse_symbol_type, Type},
};

pub struct TypeLookup {
    pub binding_power_lu: HashMap<TokenKind, i8>,

    pub led_lu: HashMap<TokenKind, fn(&mut Parser, &i8, Type) -> Type>,
    pub nod_lu: HashMap<TokenKind, fn(&mut Parser) -> Type>,
}
impl TypeLookup {
    pub fn get_led(&self, token_kind: TokenKind) -> &fn(&mut Parser, &i8, Type) -> Type {
        self.led_lu.get(&token_kind).expect(&format!(
            "led was not found for token kind: {:?}",
            token_kind
        ))
    }
    pub fn get_nod(&self, token_kind: TokenKind) -> &fn(&mut Parser) -> Type {
        self.nod_lu.get(&token_kind).expect(&format!(
            "nod was not found for token kind: {:?}",
            token_kind
        ))
    }
    pub fn get_bp(&self, token_kind: &TokenKind) -> &i8 {
        self.binding_power_lu.get(token_kind).expect(&format!(
            "bp was not found for token kind: {:?}",
            token_kind
        ))
    }
    fn led(&mut self, token_kind: TokenKind, bp: i8, function: fn(&mut Parser, &i8, Type) -> Type) {
        self.led_lu.insert(token_kind, function);
        self.binding_power_lu.insert(token_kind, bp);
    }

    fn nod(&mut self, token_kind: TokenKind, bp: i8, function: fn(&mut Parser) -> Type) {
        self.nod_lu.insert(token_kind, function);
        if bp >= -1 {
            self.binding_power_lu.insert(token_kind, bp);
        }
    }

    pub fn new() -> TypeLookup {
        let mut lookup = TypeLookup {
            binding_power_lu: HashMap::new(),
            led_lu: HashMap::new(),
            nod_lu: HashMap::new(),
        };

        // lookup.led(TokenKind::, 1, parse_assignment);
        lookup.nod(TokenKind::Identifier, 0, parse_symbol_type);
        lookup.nod(TokenKind::OpenBracket, 0, parse_array_type);

        lookup
    }
}
