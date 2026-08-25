use crate::ast::expression::Expression;
use crate::token::Token;

use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl ExpressionStatement {
    pub fn to_string(&self) -> String {
        self.expression.to_string()
    }
}
