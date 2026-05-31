use crate::ast::expression::Expression;
use crate::token::Token;

#[derive(Default, Hash, PartialOrd, Ord, Clone, PartialEq, Eq, Debug)]
pub struct LetStatement {
    pub token: Token,
    pub assignment: Expression,
}

impl LetStatement {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("let ");

        buffer.push_str(&self.assignment.to_string());

        buffer.push(';');
        buffer
    }
}
