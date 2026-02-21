use crate::{ast::expression::Expression, token::Token};



#[derive(Default, Clone, PartialEq, Eq)]
pub struct IndexExpression{
    pub token: Token,
    pub left:  Box<Expression>,
    pub right: Box<Expression>
}


impl IndexExpression{
    
    pub fn token_literal(&self) -> String{
        self.token.literal.clone()
    }

    pub fn to_string(&self) -> String{
        let mut buffer = String::new();

        buffer.push('(');
        buffer.push_str(&self.left.to_string());
        buffer.push('[');
        buffer.push_str(&self.right.to_string());
        buffer.push_str("])");
        
        buffer
    }
}
