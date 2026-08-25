use crate::ast::expression::Expression;
use crate::token::Token;

use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Option<Expression>,
}

impl ReturnStatement {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("return ");
        if let Some(val) = &self.value {
            buffer.push_str(&val.to_string());
        }

        buffer.push(';');

        buffer
    }
}
