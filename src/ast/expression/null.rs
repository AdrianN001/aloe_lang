use crate::token::Token;
use serde::{Deserialize, Serialize};

#[derive(Default, PartialOrd, Ord, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct NullExpression {
    pub token: Token,
}

impl NullExpression {
    pub fn to_string(&self) -> String {
        String::from("null")
    }
}
