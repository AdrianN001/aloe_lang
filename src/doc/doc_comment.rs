use crate::token::Token;

#[derive(Hash, PartialOrd, Ord, Clone, PartialEq, Eq, Debug)]
pub struct DocComment {
    pub raw_content: String,
}

impl DocComment {
    pub fn new_from_token(token: Token) -> Self {
        DocComment {
            raw_content: token.literal,
        }
    }
}
