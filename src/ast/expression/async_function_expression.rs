use crate::{ast::expression::Expression, token::Token};

#[derive(Default, PartialOrd, Ord, Clone, PartialEq, Eq, Hash)]
pub struct AsyncFunctionExpression {
    pub token: Token,
    pub function: Box<Expression>,
}

impl AsyncFunctionExpression {
    pub fn to_string(&self) -> String {
        format!("async {}", self.function.to_string())
    }
}
