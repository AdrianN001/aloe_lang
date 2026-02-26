use crate::{ast::expression::Expression, token::Token};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct BreakStatement {
    pub token: Token,
    pub expression: Option<Expression>,
}

impl BreakStatement {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("break ");

        if let Some(expression) = &self.expression {
            buffer.push_str(&expression.to_string());
        }

        buffer.push(';');

        buffer
    }
}
