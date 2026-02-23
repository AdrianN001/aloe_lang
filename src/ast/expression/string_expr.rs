use crate::token::Token;

#[derive(Default, Hash, PartialOrd, Ord, Clone, PartialEq, Eq)]
pub struct StringExpr{
    pub token: Token,
    pub value: String
}
