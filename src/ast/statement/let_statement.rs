
use crate::ast::expression::{Expression, identifier::Identifier};
use crate::token::Token;

#[derive(Default)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}


impl LetStatement{
    pub fn to_string(&self) -> String{
        let mut buffer = String::new(); 

        buffer.push_str("let ");
        buffer.push_str(&self.name.token.literal);
        buffer.push_str(" = ");

        //TODO add value print 

        buffer.push(';');
        buffer
    }
}
