use crate::token::token_type::TokenType;
use std::fmt;

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenType::Illegal => "ILLEGAL",
            TokenType::Eof => "EOF",
            TokenType::Comment => "#",

            TokenType::Identifier => "IDENT",
            TokenType::Integer => "INT",
            TokenType::String => "STRING",

            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Bang => "!",
            TokenType::Asterisk => "*",
            TokenType::Slash => "/",
            TokenType::QuestionMark => "?",
            TokenType::Exponent => "**",
            TokenType::Modulo => "%",

            TokenType::LogicalAnd => "&&",
            TokenType::LogicalOr => "||",
            TokenType::LogicalXor => "^",

            TokenType::BinaryAnd => "&",
            TokenType::BinaryOr => "|",
            TokenType::BinaryLeftShift => "<<",
            TokenType::BinaryRightShift => ">>",

            TokenType::IteratorAssign => "<-",
            TokenType::Dot => ".",

            TokenType::LT => "<",
            TokenType::LE => "<=",
            TokenType::GT => ">",
            TokenType::GE => ">=",
            TokenType::Eq => "==",
            TokenType::NotEq => "!=",
            TokenType::Coalescing => "??",

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
            TokenType::KwFunctionStatement => "fun",
            TokenType::KwLet => "let",
            TokenType::KwIf => "if",
            TokenType::KwElif => "elif",
            TokenType::KwElse => "else",
            TokenType::KwReturn => "return",
            TokenType::KwTrue => "true",
            TokenType::KwFalse => "false",
            TokenType::KwDefer => "defer",

            TokenType::KwStruct => "struct",

            TokenType::KwFor => "for",
            TokenType::KwBreak => "break",
            TokenType::KwContinue => "continue",

            TokenType::KwImport => "import",
            TokenType::KwFrom => "from",
            TokenType::KwInto => "into",
        };

        write!(f, "{s}")
    }
}
