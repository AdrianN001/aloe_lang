
use crate::token::Token;

#[derive(Default, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}


