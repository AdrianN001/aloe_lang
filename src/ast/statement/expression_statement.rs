use crate::token::Token;
use crate::ast::expression::Expression;

pub struct ExpressionStatement{
    pub token:      Token, 
    pub expression: Expression
}

impl ExpressionStatement{
    pub fn to_string(&self) -> String{
        return "".to_string();
    }
}
