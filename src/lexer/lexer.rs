use crate::tokens::*;
use core::str;
use std::{cmp, collections::HashMap};

struct Pattern {
    kind: TokenKind,
    value: Vec<String>,
    valueString: String,
}
impl Pattern {
    fn new(kind: TokenKind, value: &str) -> Pattern {
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
        };
    }
    pub fn onMatchNonNumber(&self, lex: &mut Lexer) {
        lex.advance(self.value.len() as u16);
        lex.push(Token {
            kind: self.kind.clone(),
            value: self.valueString.clone(),
        });
    }
}

pub struct Lexer {
    tokens: Vec<Token>,
    pub source: Vec<String>,
    pos: u16,
    blackList: Vec<TokenKind>,
}
impl Lexer {
    pub fn new(source: String, blackList: Vec<TokenKind>) -> Lexer {
        // I create those regexes by hand in code so I don't have to worry about errors in creating regexes
        let split = source.split("").map(|s| s.to_string());
        let mut source: Vec<String> = Vec::new();
        for item in split {
            if &item != "" && &item != "\r" {
                source.push(item);
            }
        }
        Lexer {
            tokens: Vec::new(),
            source,
            pos: 0,
            blackList,
        }
    }
    fn getPatterns() -> Vec<Pattern> {
        return vec![
            Pattern::new(TokenKind::NextLine, "\n"),
            Pattern::new(TokenKind::WhiteSpace, " "),
            Pattern::new(TokenKind::OpenBracket, "["),
            Pattern::new(TokenKind::CloseBracket, "]"),
            Pattern::new(TokenKind::OpenCurly, "{"),
            Pattern::new(TokenKind::CloseCurly, "}"),
            Pattern::new(TokenKind::CloseParen, ")"),
            Pattern::new(TokenKind::OpenParen, "("),
            Pattern::new(TokenKind::Equals, "=="),
            Pattern::new(TokenKind::NotEquals, "!="),
            Pattern::new(TokenKind::Assignment, "="),
            Pattern::new(TokenKind::Arrow, "->"),
            Pattern::new(TokenKind::Not, "!"),
            Pattern::new(TokenKind::LessEquals, "<="),
            Pattern::new(TokenKind::Less, "<"),
            Pattern::new(TokenKind::GreaterEquals, ">="),
            Pattern::new(TokenKind::Greater, ">"),
            Pattern::new(TokenKind::Or, "||"),
            Pattern::new(TokenKind::And, "&&"),
            Pattern::new(TokenKind::DotDot, ".."),
            Pattern::new(TokenKind::Dot, "."),
            Pattern::new(TokenKind::SemiColon, ";"),
            Pattern::new(TokenKind::Colon, ":"),
            Pattern::new(TokenKind::Question, "?"),
            Pattern::new(TokenKind::Comma, ","),
            Pattern::new(TokenKind::PlusPlus, "++"),
            Pattern::new(TokenKind::MinusMinus, "--"),
            Pattern::new(TokenKind::PlusEquals, "+="),
            Pattern::new(TokenKind::MinusEquals, "-="),
            Pattern::new(TokenKind::Plus, "+"),
            Pattern::new(TokenKind::Minus, "-"),
            Pattern::new(TokenKind::Slash, "/"),
            Pattern::new(TokenKind::Star, "*"),
            Pattern::new(TokenKind::Percent, "%"),
        ];
    }

    pub fn advance(&mut self, amount: u16) {
        self.pos += amount;
    }

    pub fn push(&mut self, token: Token) {
        if !self.blackList.contains(&token.kind) {
            self.tokens.push(token);
        }
    }
    fn at(&self) -> String {
        return self.source[self.pos as usize].to_string();
    }
    /// is at the end of file
    fn eof(&self) -> bool {
        return self.pos as usize >= self.source.len();
    }
    /// returns string containing characters from pos to pos + n
    fn nextNCharacters(&self, n: u16) -> String {
        let mut value = String::new();

        let endIndex = cmp::min(self.pos + n, self.source.len() as u16 - 1);
        for i in self.pos..endIndex {
            value += &self.source[i as usize];
        }
        return value;
    }

    pub fn reminder(&self) -> &[String] {
        return self.source.split_at(self.pos as usize).1;
    }
}

pub fn tokenize(source: String, blackList: Vec<TokenKind>) -> Vec<Token> {
    let mut lexer = Lexer::new(source, blackList);
    let patterns = Lexer::getPatterns();
    let reserved_symbols = reserved_symbols();

    while !lexer.eof() {
        let mut matched = false;
        // Check if is number
        let at = &lexer.at();

        if isNumber(at) {
            handle_number_tokenization(&mut lexer);
            continue;
        }
        if lexer.nextNCharacters(2) == "//" {
            handle_comments(&mut lexer);
            continue;
        }
        if at == "\"" {
            handle_strings(&mut lexer);
            continue;
        }
        if isSymbol(at, true) {
            handle_symbols(&mut lexer, &reserved_symbols);
            continue;
        }

        handle_standard_pattern_tokenization(&mut lexer, &patterns, &mut matched);

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

fn handle_comments(lexer: &mut Lexer) {
    println!("handle_comments");
    let mut value = String::new();
    let mut currentIndex = lexer.pos as usize;
    while currentIndex < lexer.source.len() {
        let char = &lexer.source[currentIndex];
        currentIndex += 1;
        if char == "\n" {
            break;
        }
        value += &char.to_string();
    }
    // println!("Match {:?}", &value);
    lexer.advance(value.len() as u16);
    lexer.push(Token {
        kind: TokenKind::Comment,
        value,
    });
    lexer.advance(1 as u16);
    lexer.push(Token {
        kind: TokenKind::NextLine,
        value: "\n".to_string(),
    });
}
fn handle_strings(lexer: &mut Lexer) {
    println!("handle_strings");
    let mut value = "\"".to_string();
    let mut currentIndex = (lexer.pos + 1) as usize;
    while currentIndex < lexer.source.len() {
        let char = &lexer.source[currentIndex];
        currentIndex += 1;
        value += &char.to_string();

        if char == "\"" {
            break;
        }
    }

    lexer.advance(value.len() as u16);
    lexer.push(Token {
        kind: TokenKind::String,
        value,
    });
}
fn handle_symbols(lexer: &mut Lexer, reserved_symbols: &HashMap<String, TokenKind>) {
    let mut value = String::new();
    let mut currentIndex = (lexer.pos + 1) as usize;
    value += &lexer.at();
    while currentIndex < lexer.source.len() {
        let char = &lexer.source[currentIndex];
        println!("handle_symbols {:?} {}", char, isSymbol(char, false));

        currentIndex += 1;

        if !isSymbol(char, false) {
            break;
        }
        value += &char.to_string();
    }
    lexer.advance(value.len() as u16);

    let kind = reserved_symbols.get(&value);
    match kind {
        // The division was valid
        Some(x) => lexer.push(Token {
            kind: x.clone(),
            value,
        }),
        // The division was invalid
        None => lexer.push(Token {
            kind: TokenKind::Identifier,
            value,
        }),
    }
}
fn handle_standard_pattern_tokenization(
    lexer: &mut Lexer,
    patterns: &Vec<Pattern>,
    matched: &mut bool,
) {
    for pattern in patterns {
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

            pattern.onMatchNonNumber(lexer);

            *matched = true;
            break;
        }
    }
}

pub fn reserved_symbols() -> HashMap<String, TokenKind> {
    return HashMap::from([
        ("i32".to_string(), TokenKind::I32),
        ("i64".to_string(), TokenKind::I64),
        ("f32".to_string(), TokenKind::F32),
        ("f64".to_string(), TokenKind::F64),
        ("mut".to_string(), TokenKind::Mut),
        ("str".to_string(), TokenKind::Str),
        ("var".to_string(), TokenKind::Var),
        ("const".to_string(), TokenKind::Const),
        ("enum".to_string(), TokenKind::Enum),
        ("class".to_string(), TokenKind::Class),
        ("public".to_string(), TokenKind::Public),
        ("mod".to_string(), TokenKind::Mod),
        ("new".to_string(), TokenKind::New),
        ("as".to_string(), TokenKind::As),
        ("import".to_string(), TokenKind::Import),
        ("fn".to_string(), TokenKind::Fn),
        ("in".to_string(), TokenKind::In),
        ("if".to_string(), TokenKind::If),
        ("else".to_string(), TokenKind::Else),
        ("for".to_string(), TokenKind::For),
        ("while".to_string(), TokenKind::While),
    ]);
}

const SYMBOLS: [&str; 53] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
    "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "_",
];

fn isSymbol(char: &String, beginning: bool) -> bool {
    return SYMBOLS.contains(&(char.as_str()))
        || (!beginning && NUMBERS.contains(&(char.as_str())));
}

fn handle_number_tokenization(lexer: &mut Lexer) {
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
    lexer.advance(value.len() as u16);
    lexer.push(Token {
        kind: TokenKind::Number,
        value,
    });
}
const NUMBERS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
fn isNumber(char: &String) -> bool {
    let charStr = char.as_str();
    return NUMBERS.contains(&(charStr)) || charStr == ".";
}
