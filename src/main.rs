use std::fs;

use tokens::TokenKind;

#[path = "ast/ast.rs"]
mod ast;
#[path = "parser/exprHandler.rs"]
mod exprHandler;
#[path = "parser/stmtHandler.rs"]
mod stmtHandler;

#[path = "lexer/lexer.rs"]
mod lexer;
#[path = "parser/lookup.rs"]
mod lookup;
#[path = "parser/parser.rs"]
mod parser;
#[path = "lexer/tokens.rs"]
mod tokens;
const inputFilePath: &str = "./CompileTargets/simple.lang";
fn main() {
    let content = fs::read_to_string(inputFilePath).expect("Couldn't find Input file!");
    println!("content:{:?} ------------ \n", content);

    println!("tokens:  ------------ \n");
    let tokens = lexer::tokenize(
        content,
        vec![
            TokenKind::WhiteSpace,
            TokenKind::Comment,
            TokenKind::NextLine,
        ],
    );
    let test: lookup::BindingPower;
    let mut i: u32 = 0;
    for token in &tokens {
        token.debug(i);
        i += 1;
    }

    println!("ast:  ------------ \n");

    let ast = parser::parse(tokens);
    println!("{:?}", ast);
}
