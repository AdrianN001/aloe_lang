use crate::token::Token;
use crate::token::token_type::TokenType;

#[derive(Eq, PartialEq, PartialOrd, Ord)]
pub enum OperationPrecedence {
    Lowest,

    Equals,      // ==, !=
    LessGreater, // < >
    Sum,         // +, -
    Mul,         // *, /
    Prefix,      // -x, !x,
    Member,      // string.length
    Call,        // add(x,y)
    Index,       // array[index]
}

pub fn get_precedence_of_operator(token: &Token) -> OperationPrecedence {
    match token.token_type {
        TokenType::Eq => OperationPrecedence::Equals,
        TokenType::NotEq => OperationPrecedence::Equals,

        TokenType::GT => OperationPrecedence::LessGreater,
        TokenType::LT => OperationPrecedence::LessGreater,

        TokenType::Plus => OperationPrecedence::Sum,
        TokenType::Minus => OperationPrecedence::Sum,

        TokenType::Slash => OperationPrecedence::Mul,
        TokenType::Asterisk => OperationPrecedence::Mul,

        TokenType::Dot => OperationPrecedence::Member,
        TokenType::LParen => OperationPrecedence::Call,
        TokenType::LBracket => OperationPrecedence::Index,

        _ => OperationPrecedence::Lowest,
    }
}
