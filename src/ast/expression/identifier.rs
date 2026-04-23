use crate::token::Token;

#[derive(Default, Hash, PartialOrd, Ord, Clone, PartialEq, Eq, Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
