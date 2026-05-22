use crate::{ast::expression::Expression, token::Token};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct LaunchStatement {
    pub token: Token,
    pub expr: Expression,
}

impl LaunchStatement {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str("import ");
        buffer.push_str(&self.expr.to_string());

        buffer
    }
}
