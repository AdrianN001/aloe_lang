pub mod display;
pub mod token_type;

use crate::token::token_type::TokenType;

#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line_number: usize,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String, line_number: usize) -> Self {
        Self {
            token_type,
            literal,
            line_number,
        }
    }

    pub fn simple(token_type: TokenType, literal: &str, line_number: usize) -> Self {
        Self {
            token_type,
            literal: literal.to_string(),
            line_number,
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            token_type: TokenType::Illegal,
            literal: "\0".to_string(),
            line_number: 0,
        }
    }
}
