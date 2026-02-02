
use crate::ast::expression::Expression;
use crate::token::Token;

#[derive(Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Expression,
}

impl ReturnStatement{
    pub fn to_string(&self) -> String{
        let mut buffer = String::new(); 

        buffer.push_str("return ");

        //TODO add expression 
        

        buffer.push(';');


        buffer
    }
}
