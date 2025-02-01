use std::fs;

mod lexer;
mod tokens;

const inputFilePath: &str = "./CompileTargets/Input.lang";
fn main() {
    let content = fs::read_to_string(inputFilePath).expect("Couldn't find Input file!");

    println!("content:{:?} \n ------------ \n", content);
    let testLexer = lexer::Lexer::new(content.clone());
    println!("testLexer {:?}", testLexer.source);

    let tokens = lexer::tokenize(content);
    println!("tokens:  ------------ \n");

    for token in tokens {
        token.debug();
    }
}
