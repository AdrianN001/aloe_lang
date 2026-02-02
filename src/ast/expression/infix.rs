use crate::{ast::expression::Expression, token::Token};


#[derive(Clone)]
pub struct InfixExpression{
    pub token:      Token,
    pub left:       Box<Expression>,
    pub operator:   String,
    pub right:      Box<Expression>
}


impl InfixExpression{
    pub fn to_string(&self) -> String{
        let mut buffer = String::new();
        
        buffer.push_str(&self.left.to_string());
        buffer.push_str(&self.operator);
        buffer.push_str(&self.right.to_string());

        buffer
    }
}
