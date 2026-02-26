use crate::{
    ast::{expression::Expression, statement::block_statement::BlockStatement},
    token::Token,
};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ForLoopExpression {
    pub token: Token,
    pub variable: Option<Box<Expression>>,
    pub iterator: Option<Box<Expression>>,
    pub block: BlockStatement,
}

impl ForLoopExpression {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("for ");
        if let Some(variable) = &self.variable
            && let Some(iterator) = &self.iterator
        {
            buffer.push_str(&variable.to_string());
            buffer.push_str(" <- ");
            buffer.push_str(&iterator.to_string());
        }

        buffer.push_str(&self.block.to_string());

        buffer
    }
}
