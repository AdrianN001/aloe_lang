use crate::{ast::expression::Expression, token::Token};



pub struct PrefixExpression{
    pub token:      Token, 
    pub operator:   String, 
    pub right:      Box<Expression>
}


impl PrefixExpression {
    pub fn to_string(&self) -> String{ 
        let mut buffer = String::new();

        buffer.push_str(&self.operator);
        buffer.push_str(&self.right.to_string());

        buffer
    }
}
