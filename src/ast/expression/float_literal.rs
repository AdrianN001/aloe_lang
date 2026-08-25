use crate::token::Token;
use serde::{Deserialize, Serialize};

#[derive(Default, PartialOrd, Ord, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct FloatLiteral {
    pub token: Token,
    pub integer_part: i64,
    pub float_part: u64,
}
