use crate::token::Token;
use serde::{Deserialize, Serialize};

#[derive(Default, Hash, PartialOrd, Ord, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
