use crate::token::Token;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct StringExpr{
    pub token: Token,
    pub value: String
}
