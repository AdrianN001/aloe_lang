
use crate::token::Token;

#[derive(Default)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

