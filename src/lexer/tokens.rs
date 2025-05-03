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

    EndOfFile,
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
    I16,
    U32,
    U16,
    Bool,
    Float,
    Double,
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
    pub fn debug(&self, index: u32) {
        println!("{}.    {:?} {}", index, self.kind, self.value);
    }
}
