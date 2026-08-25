use crate::token::Token;
use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, PartialOrd, Ord, Default, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}
