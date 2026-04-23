use crate::{ast::statement::Statement, token::Token};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct AsyncFunctionStatement {
    pub token: Token,
    pub function: Box<Statement>,
}

impl AsyncFunctionStatement {
    pub fn to_string(&self) -> String {
        format!("async {}", self.function.to_string())
    }
}
