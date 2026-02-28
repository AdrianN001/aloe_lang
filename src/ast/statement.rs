pub mod block_statement;
pub mod break_statement;
pub mod continue_statement;
pub mod expression_statement;
pub mod let_statement;
pub mod return_statement;

use block_statement::BlockStatement;
use expression_statement::ExpressionStatement;
use let_statement::LetStatement;
use return_statement::ReturnStatement;


use crate::ast::statement::{
    break_statement::BreakStatement, continue_statement::ContinueStatement,
};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
}

impl Statement {
    pub fn token_literal(&self) -> &str {
        match self {
            Statement::Let(s) => &s.token.literal,
            Statement::Return(s) => &s.token.literal,
            Statement::Expression(s) => &s.token.literal,
            Statement::Block(s) => &s.token.literal,
            Statement::Break(s) => &s.token.literal,
            Statement::Continue(s) => &s.token.literal,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Statement::Let(s) => s.to_string(),
            Statement::Return(s) => s.to_string(),
            Statement::Expression(s) => s.to_string(),
            Statement::Block(s) => s.to_string(),
            Statement::Break(s) => s.to_string(),
            Statement::Continue(s) => s.to_string(),
        }
    }
}
