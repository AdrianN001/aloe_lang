use crate::{ast::expression::Expression, token::Token};

#[derive(Clone, Hash, PartialOrd, Ord, Default, PartialEq, Eq, Debug)]
pub struct AwaitExpression {
    pub token: Token,
    pub expr: Box<Expression>,
}

impl AwaitExpression {
    pub fn to_string(&self) -> String {
        format!("await {}", self.expr.to_string())
    }
}
