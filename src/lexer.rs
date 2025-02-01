use core::str;

use crate::tokens::*;

struct Pattern {
    number: bool,
    kind: TokenKind,
    value: Vec<String>,
    valueString: String,
}
impl Pattern {
    fn new(kind: TokenKind, value: &str, number: bool) -> Pattern {
        let split = value.split("").map(|s| s.to_string());
        let mut source: Vec<String> = Vec::new();
        for item in split {
            if &item != "" {
                source.push(item);
            }
        }
        return Pattern {
            value: source,
            valueString: value.to_string(),
            kind,
            number,
        };
    }
    pub fn onMatchNonNumber(&self, lex: &mut Lexer) {
        lex.advance(self.value.len() as u16);
        lex.push(Token {
            kind: self.kind.clone(),
            value: self.valueString.clone(),
        });
    }
    pub fn onMatchNumber(&self, lex: &mut Lexer, value: String) {
        lex.advance(value.len() as u16);
        lex.push(Token {
            kind: self.kind.clone(),
            value,
        });
    }
}

pub struct Lexer {
    tokens: Vec<Token>,
    pub source: Vec<String>,
    pos: u16,
}
impl Lexer {
    pub fn new(source: String) -> Lexer {
        // I create those regexes by hand in code so I don't have to worry about errors in creating regexes
        let split = source.split("").map(|s| s.to_string());
        let mut source: Vec<String> = Vec::new();
        for item in split {
            if &item != "" {
                source.push(item);
            }
        }
        Lexer {
            tokens: Vec::new(),
            source,
            pos: 0,
        }
    }
    fn getPatterns() -> Vec<Pattern> {
        return vec![
            Pattern::new(TokenKind::Number, " ", true),
            Pattern::new(TokenKind::WhiteSpace, " ", false),
            Pattern::new(TokenKind::OpenBracket, "[", false),
            Pattern::new(TokenKind::CloseBracket, "]", false),
            Pattern::new(TokenKind::OpenCurly, "{", false),
            Pattern::new(TokenKind::CloseCurly, "}", false),
            Pattern::new(TokenKind::CloseParen, ")", false),
            Pattern::new(TokenKind::OpenParen, "(", false),
            Pattern::new(TokenKind::Equals, "==", false),
            Pattern::new(TokenKind::NotEquals, "!=", false),
            Pattern::new(TokenKind::Assignment, "=", false),
            Pattern::new(TokenKind::Arrow, "->", false),
            Pattern::new(TokenKind::Not, "!", false),
            Pattern::new(TokenKind::LessEquals, "<=", false),
            Pattern::new(TokenKind::Less, "<", false),
            Pattern::new(TokenKind::GreaterEquals, ">=", false),
            Pattern::new(TokenKind::Greater, ">", false),
            Pattern::new(TokenKind::Or, "||", false),
            Pattern::new(TokenKind::And, "&&", false),
            Pattern::new(TokenKind::DotDot, "..", false),
            Pattern::new(TokenKind::Dot, ".", false),
            Pattern::new(TokenKind::SemiColon, ";", false),
            Pattern::new(TokenKind::Colon, ":", false),
            Pattern::new(TokenKind::Question, "?", false),
            Pattern::new(TokenKind::Comma, ",", false),
            Pattern::new(TokenKind::PlusPlus, "++", false),
            Pattern::new(TokenKind::MinusMinus, "--", false),
            Pattern::new(TokenKind::PlusEquals, "+=", false),
            Pattern::new(TokenKind::MinusEquals, "-=", false),
            Pattern::new(TokenKind::Plus, "+", false),
            Pattern::new(TokenKind::Minus, "-", false),
            Pattern::new(TokenKind::Slash, "/", false),
            Pattern::new(TokenKind::Star, "*", false),
            Pattern::new(TokenKind::Percent, "%", false),
        ];
    }

    pub fn advance(&mut self, amount: u16) {
        self.pos += amount;
    }

    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }
    fn at(&self) -> String {
        return self.source[self.pos as usize].to_string();
    }
    /// is at the end of file
    fn eof(&self) -> bool {
        return self.pos as usize >= self.source.len();
    }

    pub fn reminder(&self) -> &[String] {
        return self.source.split_at(self.pos as usize).1;
    }
}

pub fn tokenize(source: String) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    let patterns = Lexer::getPatterns();
    while !lexer.eof() {
        let mut matched = false;
        for pattern in &patterns {
            if pattern.number {
                if !isNumber(&lexer.source[lexer.pos as usize]) {
                    continue;
                }
                let mut value = String::new();
                let mut currentIndex = lexer.pos as usize;
                loop {
                    let char = &lexer.source[currentIndex];
                    currentIndex += 1;
                    if !isNumber(char) {
                        break;
                    }
                    value += &char.to_string();
                }

                // println!("Match {:?}", &value);
                pattern.onMatchNumber(&mut lexer, value);

                matched = true;
                break;
            }
            let mut equal = true;

            // not a number
            for i in 0..pattern.value.len() {
                /*  println!(
                    "not a number {:?} {:?} {:?} {}",
                    pattern.kind,
                    pattern.value[i],
                    lexer.source[i + lexer.pos as usize],
                    pattern.value[i] == lexer.source[i + lexer.pos as usize]
                ); */

                if pattern.value[i] != lexer.source[i + lexer.pos as usize] {
                    equal = false;
                    break;
                }
            }
            if equal {
                // println!("Match {:?}", pattern.valueString);

                pattern.onMatchNonNumber(&mut lexer);

                matched = true;
                break;
            }
        }

        if !matched {
            panic!(
                "Lexer Error -> unrecognized token near {:?}",
                lexer.reminder()
            );
        }
    }
    lexer.push(Token {
        kind: TokenKind::Eof,
        value: "Eof".to_string(),
    });
    return lexer.tokens;
}
const NUMBERS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
fn isNumber(char: &String) -> bool {
    return NUMBERS.contains(&(char.as_str()));
}
