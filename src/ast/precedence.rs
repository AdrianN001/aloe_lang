use crate::token::Token;
use crate::token::token_type::TokenType;

#[derive(Eq, PartialEq, PartialOrd, Ord)]
pub enum OperationPrecedence {
    Lowest,

    Assignment,  // =
    Coalescing,  // ??
    LogicalOr,   // ||
    LogicalAnd,  // &&
    Equals,      // ==, !=
    LessGreater, // <, >, <=, >=
    BinaryOr,    // |
    Xor,         // ^
    BinaryAnd,   // &
    BinaryShift, // <<, >>
    Sum,         // +, -
    Mul,         // *, /
    Prefix,      // -x, !x,
    Exponent,    // **

    Index,  // array[index]
    Member, // string.length
    Call,   // add(x,y)
}

pub fn get_precedence_of_operator(token: &Token) -> OperationPrecedence {
    match token.token_type {
        TokenType::Assign => OperationPrecedence::Assignment,
        TokenType::Coalescing => OperationPrecedence::Coalescing,

        TokenType::LogicalOr => OperationPrecedence::LogicalOr,
        TokenType::LogicalAnd => OperationPrecedence::LogicalAnd,

        TokenType::Eq => OperationPrecedence::Equals,
        TokenType::NotEq => OperationPrecedence::Equals,

        TokenType::GT => OperationPrecedence::LessGreater,
        TokenType::GE => OperationPrecedence::LessGreater,
        TokenType::LE => OperationPrecedence::LessGreater,
        TokenType::LT => OperationPrecedence::LessGreater,

        TokenType::BinaryOr => OperationPrecedence::BinaryOr,
        TokenType::LogicalXor => OperationPrecedence::Xor,
        TokenType::BinaryAnd => OperationPrecedence::BinaryAnd,
        TokenType::BinaryRightShift | TokenType::BinaryLeftShift => {
            OperationPrecedence::BinaryShift
        }

        TokenType::Plus => OperationPrecedence::Sum,
        TokenType::Minus => OperationPrecedence::Sum,

        TokenType::Modulo => OperationPrecedence::Mul,
        TokenType::Slash => OperationPrecedence::Mul,
        TokenType::Asterisk => OperationPrecedence::Mul,

        TokenType::Exponent => OperationPrecedence::Exponent,

        TokenType::Dot => OperationPrecedence::Member,
        TokenType::LParen => OperationPrecedence::Call,
        TokenType::LBracket => OperationPrecedence::Index,

        _ => OperationPrecedence::Lowest,
    }
}
