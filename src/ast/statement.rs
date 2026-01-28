
pub mod let_statement;
pub mod return_statement;

use let_statement::LetStatement;
use return_statement::ReturnStatement;

pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}


impl Statement {
    pub fn token_literal(&self) -> &str {
        match self {
            Statement::Let(s) => &s.token.literal,
            Statement::Return(s) => &s.token.literal,
        }
    }
}


