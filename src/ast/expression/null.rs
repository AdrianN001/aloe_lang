use crate::token::Token;

#[derive(Default, PartialOrd, Ord, Clone, PartialEq, Eq, Hash, Debug)]
pub struct NullExpression {
    pub token: Token,
}

impl NullExpression {
    pub fn to_string(&self) -> String {
        String::from("null")
    }
}
