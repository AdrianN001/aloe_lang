use crate::token::Token;

#[derive(Clone, Hash, PartialOrd, Ord, Default, PartialEq, Eq, Debug)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}
