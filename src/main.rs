use std::fs;

use tokens::TokenKind;

#[path = "lexer/lexer.rs"]
mod lexer;

#[path = "parser/parser.rs"]
mod parser;

#[path = "codeGen/code_gen.rs"]
mod code_gen;
#[path = "parser/expression.rs"]
mod expression;
#[path = "parser/lookup.rs"]
mod lookup;
// #[path = "parser/statement.rs"]
// mod statement;
#[path = "lexer/tokens.rs"]
mod tokens;
const INPUT_FILE_PATH: &str = "./CompileTargets/simple.lang";
const OUTPUT_FILE_PATH: &str = "./CompileTargets/Output.c";

fn main() {
    let content = fs::read_to_string(INPUT_FILE_PATH).expect("Couldn't find input file!");
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
    let mut i: u32 = 0;
    for token in &tokens {
        token.debug(i);
        i += 1;
    }

    println!("ast:  ------------ \n");

    let expressions = parser::parse(tokens);
    println!("{:?}", expressions);

    println!("Output:  ------------ \n");

    let output_code = code_gen::convert_expressions_to_code(expressions);
    fs::write(OUTPUT_FILE_PATH, &output_code).expect("Couldn't find output file!");

    println!("{}", output_code);
}
