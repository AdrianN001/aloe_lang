use crate::ast::expression::{Expression, identifier::Identifier};
use crate::token::Token;

#[derive(Default, Hash, PartialOrd, Ord, Clone, PartialEq, Eq)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl LetStatement {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("let ");
        buffer.push_str(&self.name.token.literal);
        buffer.push_str(" = ");

        buffer.push_str(&self.value.to_string());

        buffer.push(';');
        buffer
    }
}
