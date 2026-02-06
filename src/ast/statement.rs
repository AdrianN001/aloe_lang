
pub mod let_statement;
pub mod return_statement;
pub mod expression_statement;
pub mod block_statement;


use block_statement::BlockStatement;
use let_statement::LetStatement;
use return_statement::ReturnStatement;
use expression_statement::ExpressionStatement;

#[derive(Clone, PartialEq, Eq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement)
}


impl Statement {
    pub fn token_literal(&self) -> &str {
        match self {
            Statement::Let(s) => &s.token.literal,
            Statement::Return(s) => &s.token.literal,
            Statement::Expression(s) => &s.token.literal,
            Statement::Block(s) => &s.token.literal,
        }
    }

    pub fn to_string(&self) -> String{
        match self{
            Statement::Let(s) => s.to_string(),
            Statement::Return(s) => s.to_string(),
            Statement::Expression(s) => s.to_string(),
            Statement::Block(s) => s.to_string()
        }
    }
}


