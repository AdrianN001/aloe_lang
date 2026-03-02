use crate::{
    ast::{expression::Expression, statement::block_statement::BlockStatement},
    token::Token,
};

#[derive(Clone, Hash, PartialOrd, Ord, Default, PartialEq, Eq)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<Expression>,

    pub alternatives: Vec<(Box<Expression>, BlockStatement)>,

    pub consequence: BlockStatement,
    pub else_block: Option<BlockStatement>,
}

impl IfExpression {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("if");
        buffer.push_str(&self.condition.to_string());
        buffer.push(' ');
        buffer.push_str(&self.consequence.to_string());

        match &self.else_block {
            Some(alternative) => {
                buffer.push_str("else ");
                buffer.push_str(&alternative.to_string());
            }
            None => {}
        }

        buffer
    }
}
