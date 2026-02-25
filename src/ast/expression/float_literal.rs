use crate::token::Token;


#[derive(Default, PartialOrd, Ord, Clone, PartialEq, Eq, Hash)]
pub struct FloatLiteral{
    pub token: Token,
    pub integer_part: i32,
    pub float_part: u32,
}
