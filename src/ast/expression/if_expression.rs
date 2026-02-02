use crate::{ast::{expression::Expression, statement::block_statement::BlockStatement}, token::Token};


#[derive(Clone, Default)]
pub struct IfExpression{
    pub token: Token,
    pub condition: Box<Expression>,
    
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>
}

impl IfExpression{
    pub fn to_string(&self) -> String{
        let mut buffer = String::new();
        
        buffer.push_str("if");
        buffer.push_str(&self.condition.to_string());
        buffer.push(' ');
        buffer.push_str(&self.consequence.to_string());
        
        match &self.alternative{
            Some(alternative) => {
                buffer.push_str("else ");
                buffer.push_str(&alternative.to_string());
            },
            None => {}
        }



        buffer
    }
}
