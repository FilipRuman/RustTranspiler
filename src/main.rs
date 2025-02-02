use std::fs;

use tokens::TokenKind;

#[path = "lexer/lexer.rs"]
mod lexer;
#[path = "lexer/tokens.rs"]
mod tokens;

const inputFilePath: &str = "./CompileTargets/simple.lang";
fn main() {
    let content = fs::read_to_string(inputFilePath).expect("Couldn't find Input file!");
    println!("content:{:?} ------------ \n", content);

    println!("tokens:  ------------ \n");
    let tokens = lexer::tokenize(content, vec![TokenKind::WhiteSpace]);

    for token in tokens {
        token.debug();
    }
}
