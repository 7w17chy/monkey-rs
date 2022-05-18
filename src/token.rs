#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    // special tokens
    Illegal,
    EOF,

    // keywords/identifiers
    Ident(String),
    Function,
    Let,
    If,
    Else,
    Return,

    // literal values
    Int(u32),
    Boolean(bool),

    // operators/special characters
    Assign,
    Plus,
    Minus,
    Comma,
    Div, // /
    Mul, // *
    Dot,
    Semicolon,
    LessThan, // <
    MoreThan, // >
    Bang, // !
    DoesntEqual, // !=
    Equals, // ==

    // parenthesis
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }
    LBracket, // [
    RBracket, // ]
}
