
use crate::ast::expression::Expression;
use crate::token::Token;

pub struct ReturnStatement {
    pub token: Token,
    pub value: Expression,
}


