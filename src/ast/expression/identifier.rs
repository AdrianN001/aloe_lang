
use crate::token::Token;

#[derive(Default, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

