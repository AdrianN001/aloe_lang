
use crate::ast::expression::{Expression, identifier::Identifier};
use crate::token::Token;

#[derive(Default)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

