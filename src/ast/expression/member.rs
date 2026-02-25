use crate::{ast::expression::Expression, object::ObjectRef, token::Token};


#[derive(Default, PartialOrd, Ord, Clone, PartialEq, Eq, Hash)]
pub struct MemberExpression{
    pub token:          Token,
    
    pub member_name:    String,
    pub left:           Box<Expression>,
    pub right:          Box<Expression>
}


impl MemberExpression{

    pub fn to_string(&self) -> String{ 
        let mut buffer = String::new();

        buffer.push_str(&self.left.to_string());
        buffer.push('.');
        buffer.push_str(&self.right.to_string());

        buffer
    }
}
