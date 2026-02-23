use crate::token::Token;
use crate::ast::expression::Expression;

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ExpressionStatement{
    pub token:      Token, 
    pub expression: Expression
}

impl ExpressionStatement{
    pub fn to_string(&self) -> String{
        self.expression.to_string()
    }
}
