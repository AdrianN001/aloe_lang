use crate::{
    ast::{expression::Expression, statement::block_statement::BlockStatement},
    token::Token,
};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct WhileLoopExpression {
    pub token: Token,
    pub condition: Option<Box<Expression>>,
    pub block: BlockStatement,
}

impl WhileLoopExpression {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("while ");
        if let Some(condition) = &self.condition {
            buffer.push_str(&condition.to_string());
        }
        buffer.push_str("{\n");

        buffer.push_str(&self.block.to_string());

        buffer.push_str("}\n");
        buffer
    }
}
