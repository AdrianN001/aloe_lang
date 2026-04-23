use crate::token::Token;

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct ContinueStatement {
    pub token: Token,
}

impl ContinueStatement {
    pub fn to_string(&self) -> String {
        "continue;".into()
    }
}
