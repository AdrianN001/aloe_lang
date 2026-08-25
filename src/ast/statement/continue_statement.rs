use crate::token::Token;

use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ContinueStatement {
    pub token: Token,
}

impl ContinueStatement {
    pub fn to_string(&self) -> String {
        "continue;".into()
    }
}
