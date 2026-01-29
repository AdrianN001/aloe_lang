
pub mod let_statement;
pub mod return_statement;
pub mod expression_statement;

use let_statement::LetStatement;
use return_statement::ReturnStatement;

use expression_statement::ExpressionStatement;

pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement)
}


impl Statement {
    pub fn token_literal(&self) -> &str {
        match self {
            Statement::Let(s) => &s.token.literal,
            Statement::Return(s) => &s.token.literal,
            Statement::Expression(s) => &s.token.literal,
        }
    }

    pub fn to_string(&self) -> String{
        match self{
            Statement::Let(s) => s.to_string(),
            Statement::Return(s) => s.to_string(),
            Statement::Expression(s) => s.to_string(),
        }
    }
}


