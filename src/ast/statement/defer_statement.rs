use crate::{ast::expression::Expression, token::Token};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct DeferStatement {
    pub token: Token,
    pub expression: Expression,
}

impl DeferStatement {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("defer ");
        buffer.push_str(&self.expression.to_string());

        buffer
    }
}
