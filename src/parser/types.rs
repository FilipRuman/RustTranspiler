use crate::{
    parser::{self, Parser},
    tokens::TokenKind,
};
#[derive(Clone, Debug)]
pub enum Type {
    Symbol(String),
    Array {
        left_type: Box<Type>,
        dimensions: usize,
    },
}

pub fn parse_symbol_type(parser: &mut Parser) -> Type {
    debug_type("parse_symbol_type");
    return Type::Symbol(parser.expect(&TokenKind::Identifier).value.to_owned());
}

pub fn parse_array_type(parser: &mut Parser, bp: &i8, left: Type) -> Type {
    debug_type("parse_array_type");
    parser.expect(&TokenKind::OpenBracket);
    let mut dimensions = 0;
    while parser.current_token_kind() == &TokenKind::Comma {
        parser.advance();
        dimensions += 1;
    }
    parser.expect(&TokenKind::CloseBracket);

    return Type::Array {
        left_type: Box::new(left),
        dimensions,
    };
}
pub fn parse_type(parser: &mut Parser, bp: &i8) -> Type {
    debug_type(" type:");
    let nod = parser.current_token();
    // let mut to_debug = format!("parse_expr: nod:{:?} bp:{} ", nod, bp,);
    let mut left = parser.type_lookup.get_nod(nod.kind)(parser);

    // to_debug += &format!(
    //     "current_token_kind:{:?} current_bp:{}",
    //     parser.current_token_kind(),
    //     parser.lookup.get_bp(&TokenKind::Plus)
    // );
    // debug_expression(&to_debug);

    while parser.current_bp() > bp {
        let led = parser.current_token().kind.clone();
        let led_fn = parser.type_lookup.get_led(led);

        // debug_expression(&format!("expr led call: led:{:?} ->>:", led));

        left = led_fn(parser, &parser.current_bp().to_owned(), left);

        // debug_expression(&format!(
        //     "expr while loop: current kind{:?} current bp:{} bp:{}",
        //     parser.current_token_kind(),
        //     parser.current_bp(),
        //     bp
        // ));
    }
    return left;
}
const SHOW_TYPE_DEBUG: bool = true;
fn debug_type(text: &str) {
    if !SHOW_TYPE_DEBUG {
        return;
    }

    println!("{}", text);
}
