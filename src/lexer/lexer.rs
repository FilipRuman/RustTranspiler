use crate::tokens::*;
use core::str;
use std::{cmp, collections::HashMap, usize};

struct Pattern {
    kind: TokenKind,
    value: Vec<String>,
    value_string: String,
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
            value_string: value.to_string(),
            kind,
        };
    }
    pub fn on_match_non_number(&self, lex: &mut Lexer) {
        lex.advance(self.value.len() as u16);
        lex.push(Token {
            kind: self.kind.clone(),
            value: self.value_string.clone(),
            line: lex.current_line,
        });
    }
}

pub struct Lexer {
    tokens: Vec<Token>,
    pub source: Vec<String>,
    pos: u16,
    current_line: u16,
    black_list: Vec<TokenKind>,
}
impl Lexer {
    pub fn new(source: String, black_list: Vec<TokenKind>) -> Lexer {
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
            black_list,
            current_line: 0,
        }
    }
    fn get_patterns() -> Vec<Pattern> {
        return vec![
            Pattern::new(TokenKind::NextLine, "\n"),
            Pattern::new(TokenKind::Tab, "\t"),
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
        for i in self.pos..self.pos + amount {
            if self.source[i as usize] == "\n" {
                self.current_line += 1;
            }
        }

        self.pos += amount;
    }

    pub fn push(&mut self, token: Token) {
        if !self.black_list.contains(&token.kind) {
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
    fn next_ncharacters(&self, n: u16) -> String {
        let mut value = String::new();

        let end_index = cmp::min(self.pos + n, self.source.len() as u16 - 1);
        for i in self.pos..end_index {
            value += &self.source[i as usize];
        }
        return value;
    }

    pub fn reminder(&self) -> &[String] {
        return self.source.split_at(self.pos as usize).1;
    }
}

pub fn tokenize(source: String, black_list: Vec<TokenKind>) -> Vec<Token> {
    let mut lexer = Lexer::new(source, black_list);
    let patterns = Lexer::get_patterns();
    let reserved_symbols = reserved_symbols();

    while !lexer.eof() {
        let mut matched = false;
        // Check if is number
        let at = &lexer.at();

        if is_number(at) {
            handle_number_tokenization(&mut lexer);
            continue;
        }
        if lexer.next_ncharacters(2) == "//" {
            handle_comments(&mut lexer);
            continue;
        }
        if at == "\"" || at == "$" {
            handle_strings(&mut lexer);
            continue;
        }
        if is_symbol(at, true) {
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
        kind: TokenKind::EndOfFile,
        value: "Eof".to_string(),
        line: lexer.current_line,
    });
    return lexer.tokens;
}

fn handle_comments(lexer: &mut Lexer) {
    // println!("handle_comments");
    let mut value = String::new();
    let mut current_index = lexer.pos as usize;
    while current_index < lexer.source.len() {
        let char = &lexer.source[current_index];
        current_index += 1;
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
        line: lexer.current_line,
    });
    lexer.advance(1 as u16);
    lexer.push(Token {
        kind: TokenKind::NextLine,
        value: "\n".to_string(),
        line: lexer.current_line,
    });
}
fn handle_strings(lexer: &mut Lexer) {
    let mut current_index = (lexer.pos + 1) as usize;
    let format_symbol = if &lexer.source[lexer.pos as usize] == "$" {
        current_index += 1;
        "$"
    } else {
        ""
    };

    let mut value = format!("{}\"", format_symbol);

    while current_index < lexer.source.len() {
        let char = &lexer.source[current_index];
        current_index += 1;
        value += &char.to_string();

        if char == "\"" {
            break;
        }
    }

    println!("handle_strings {value}");
    lexer.advance(value.len() as u16);
    lexer.push(Token {
        kind: TokenKind::String,
        value,
        line: lexer.current_line,
    });
}
fn handle_symbols(lexer: &mut Lexer, reserved_symbols: &HashMap<String, TokenKind>) {
    let mut value = String::new();
    let mut current_index = (lexer.pos + 1) as usize;
    value += &lexer.at();
    while current_index < lexer.source.len() {
        let char = &lexer.source[current_index];
        // println!("handle_symbols {:?} {}", char, isSymbol(char, false));

        current_index += 1;

        if !is_symbol(char, false) {
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
            line: lexer.current_line,
        }),
        // The division was invalid
        None => lexer.push(Token {
            kind: TokenKind::Identifier,
            value,
            line: lexer.current_line,
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

            pattern.on_match_non_number(lexer);

            *matched = true;
            break;
        }
    }
}

pub fn reserved_symbols() -> HashMap<String, TokenKind> {
    return HashMap::from([
        ("mut".to_string(), TokenKind::Mut),
        ("out".to_string(), TokenKind::Out),
        ("let".to_string(), TokenKind::Let),
        ("const".to_string(), TokenKind::Const),
        ("enum".to_string(), TokenKind::Enum),
        ("class".to_string(), TokenKind::Class),
        ("pub".to_string(), TokenKind::Pub),
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
        ("return".to_string(), TokenKind::Return),
    ]);
}

const SYMBOLS: [&str; 55] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
    "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "_", "\\", "'",
];

fn is_symbol(char: &String, beginning: bool) -> bool {
    return SYMBOLS.contains(&(char.as_str()))
        || (!beginning && NUMBERS.contains(&(char.as_str())));
}

fn handle_number_tokenization(lexer: &mut Lexer) {
    let mut value = String::new();
    let mut current_index = lexer.pos as usize;
    loop {
        let char = &lexer.source[current_index];
        current_index += 1;

        if (!is_number(char) && char != ".") ||/* makes iterators inside numbers work like: 0..100*/
        (char == "." &&  /* next char because the index is increased just before */&lexer.source[current_index ] == ".")
        {
            break;
        }
        value += &char.to_string();
    }
    // println!("Match {:?}", &value);
    lexer.advance(value.len() as u16);
    lexer.push(Token {
        kind: TokenKind::Number,
        value,
        line: lexer.current_line,
    });
}
const NUMBERS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
fn is_number(char: &String) -> bool {
    let char_str = char.as_str();
    return NUMBERS.contains(&(char_str));
}
