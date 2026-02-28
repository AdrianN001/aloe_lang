use crate::{ast::expression::Expression, token::Token};



#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ValueAssignExpression{
    pub token: Token,
    pub left: Box<Expression>, 
    pub right: Box<Expression>
}

impl ValueAssignExpression {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str(&self.left.to_string());
        buffer.push_str(" = ");
        buffer.push_str(&self.right.to_string());

        buffer.push(';');
        buffer
    }
}
