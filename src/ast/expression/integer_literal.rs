use crate::token::Token;

#[derive(Default, Hash, PartialOrd, Ord, Clone, PartialEq, Eq, Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}
