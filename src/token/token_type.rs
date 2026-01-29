

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType{
    Illegal, 
    Eof,

    // Identifiers + literals 
    Identifier, 
    Integer, 

    // Operators 
    Assign,
    Plus,
    Minus, 
    Bang, 
    Asterisk,
    Slash, 

    Eq, 
    NotEq,
    LT,     // < 
    GT,     // >

    // Delimiters 
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    KwFunction,
    KwLet,
    KwTrue, 
    KwFalse,
    KwIf, 
    KwElse,
    KwReturn
}

pub fn lookup_identifiers(identifier: &str) -> TokenType{
    match identifier{
        "fn" => TokenType::KwFunction,
        "let" => TokenType::KwLet,
        "true" => TokenType::KwTrue,
        "false" => TokenType::KwFalse,
        "if" => TokenType::KwIf,
        "else" => TokenType::KwElse,
        "return" => TokenType::KwReturn,

        _ => TokenType::Identifier
    }
}
