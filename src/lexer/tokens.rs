#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    WhiteSpace,
    OpenBracket,
    CloseBracket,
    OpenCurly,
    CloseCurly,
    CloseParen,
    OpenParen,

    DotDot,
    Dot,
    Coma,
    Arrow,

    Equals,
    NotEquals,
    Assignment,
    Not,
    LessEquals,
    Less,
    GreaterEquals,
    Greater,
    Or,
    And,
    SemiColon,
    Colon,
    Question,
    Comma,
    PlusEquals,
    MinusEquals,

    PlusPlus,
    MinusMinus,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    Eof,
    Number,
    NextLine,
    Comment,
    String,
    Identifier,

    Var,
    Const,
    Fn,
    Enum,
    Class,
    Public,
    Mod,
    As,
    New,
    Import,
    For,
    In,
    If,
    Else,
    While,

    I32,
    I64,
    F32,
    F64,
    Str,
    Mut,
}
#[derive(Debug, Clone)]

pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub line: u16,
}
impl Token {
    pub fn debug(&self) {
        println!("{:?} {}", self.kind, self.value);
    }
}
