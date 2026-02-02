
pub mod identifier;
pub mod integer_literal;
pub mod prefix_expression;
pub mod infix;

use identifier::{Identifier};
use integer_literal::IntegerLiteral;
use prefix_expression::PrefixExpression;
use infix::InfixExpression;

#[derive(Default, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),

    #[default]
    InvalidExpression
}

impl Expression {
    pub fn token_literal(&self) -> &str {
        match self {
            Expression::Identifier(i) => &i.token.literal,
            Expression::IntegerLiteral(i) => &i.token.literal,
            _ => ""
        }
    }

    pub fn to_string(&self) -> String{
        match self{ 
            Expression::Prefix(s) => s.to_string(),
            Expression::Infix(s) => s.to_string(),
            other => other.token_literal().to_string()
        }
    }
}

