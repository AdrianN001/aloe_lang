use crate::ast::expression::Expression;
use crate::token::Token;

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Expression,
}

impl ReturnStatement {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("return ");

        buffer.push_str(&self.value.to_string());

        buffer.push(';');

        buffer
    }
}
