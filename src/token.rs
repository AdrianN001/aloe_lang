pub mod token_type;

use crate::token::token_type::TokenType;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Token{
    pub token_type: TokenType,
    pub literal:    String
}

impl Token{
    pub fn new(token_type: TokenType, literal: String) -> Self{
        Self{
            token_type,
            literal
        }
    }

    pub fn simple(token_type: TokenType, literal: &str) -> Self{
        Self{
            token_type, 
            literal: literal.to_string()
        }
    }
}


impl Default for Token{
    fn default() -> Self {
        Self{
            token_type: TokenType::Illegal,
            literal: "\0".to_string()
        }
    }
}
