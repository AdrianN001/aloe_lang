#[derive(Debug, Hash, PartialOrd, Ord, Clone, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Identifier,
    Integer,
    String,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Eq,
    NotEq,
    LT, // <
    GT, // >

    IteratorAssign, // <-

    Dot,
    Comment, // #

    // Delimiters
    Colon,
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // Keywords
    KwFunction,
    KwLet,
    KwTrue,
    KwFalse,
    KwIf,
    KwElse,
    KwReturn,

    KwFor,
    KwBreak,
    KwContinue,
}

pub fn lookup_identifiers(identifier: &str) -> TokenType {
    match identifier {
        "fn" => TokenType::KwFunction,
        "let" => TokenType::KwLet,
        "true" => TokenType::KwTrue,
        "false" => TokenType::KwFalse,
        "if" => TokenType::KwIf,
        "else" => TokenType::KwElse,
        "return" => TokenType::KwReturn,
        "for" => TokenType::KwFor,
        "break" => TokenType::KwBreak,
        "continue" => TokenType::KwContinue,

        _ => TokenType::Identifier,
    }
}
