
pub mod identifier;
pub mod integer_literal;

use identifier::{Identifier};
use integer_literal::IntegerLiteral;

#[derive(Default)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),

    #[default]
    InvalidExpression
}

impl Expression {
    pub fn token_literal(&self) -> &str {
        match self {
            Expression::Identifier(i) => &i.token.literal,
            Expression::IntegerLiteral(i) => &i.token.literal,
            _ => &""
        }
    }
}

