use crate::token::token_type::TokenType;
use std::fmt;

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenType::Illegal => "ILLEGAL",
            TokenType::Eof => "EOF",

            TokenType::Identifier => "IDENT",
            TokenType::Integer => "INT",
            TokenType::String => "STRING",

            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Bang => "!",
            TokenType::Asterisk => "*",
            TokenType::Slash => "/",

            TokenType::Dot => ".",

            TokenType::LT => "<",
            TokenType::GT => ">",
            TokenType::Eq => "==",
            TokenType::NotEq => "!=",

            TokenType::Colon => ":",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",

            TokenType::LParen => "(",
            TokenType::RParen => ")",
            TokenType::LBrace => "{",
            TokenType::RBrace => "}",
            TokenType::LBracket => "[",
            TokenType::RBracket => "]",

            TokenType::KwFunction => "fn",
            TokenType::KwLet => "let",
            TokenType::KwIf => "if",
            TokenType::KwElse => "else",
            TokenType::KwReturn => "return",
            TokenType::KwTrue => "true",
            TokenType::KwFalse => "false",
        };

        write!(f, "{s}")
    }
}
