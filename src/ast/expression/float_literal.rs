use crate::token::Token;

#[derive(Default, PartialOrd, Ord, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FloatLiteral {
    pub token: Token,
    pub integer_part: i64,
    pub float_part: u64,
}
