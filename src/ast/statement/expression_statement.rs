use crate::ast::expression::Expression;
use crate::token::Token;

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl ExpressionStatement {
    pub fn to_string(&self) -> String {
        self.expression.to_string()
    }
}
