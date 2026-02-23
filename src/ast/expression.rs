
pub mod identifier;
pub mod integer_literal;
pub mod prefix_expression;
pub mod infix;
pub mod boolean;
pub mod if_expression;
pub mod function_expression;
pub mod call_expression;
pub mod string_expr;
pub mod array_literal;
pub mod index_expression;
pub mod hash_map_literal;

use hash_map_literal::HashMapLiteral;
use index_expression::IndexExpression;
use array_literal::ArrayLiteral;
use string_expr::StringExpr;
use call_expression::CallExpression;
use function_expression::FunctionExpression;
use if_expression::IfExpression;
use boolean::Boolean;
use identifier::{Identifier};
use integer_literal::IntegerLiteral;
use prefix_expression::PrefixExpression;
use infix::InfixExpression;


#[derive(Default, PartialOrd, Ord, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    Bool(Boolean),
    Function(FunctionExpression),
    Call(CallExpression),
    String(StringExpr),
    Array(ArrayLiteral),
    Index(IndexExpression),
    HashMapLiteral(HashMapLiteral),

    If(IfExpression),

    #[default]
    InvalidExpression
}

impl Expression {
    pub fn token_literal(&self) -> &str {
        match self {
            Expression::Bool(i) => &i.token.literal,
            Expression::Identifier(i) => &i.token.literal,
            Expression::IntegerLiteral(i) => &i.token.literal,
            Expression::String(str_expr) => &str_expr.value,
            _ => ""
        }
    }

    pub fn to_string(&self) -> String{
        match self{ 
            Expression::Prefix(s) => s.to_string(),
            Expression::Infix(s) => s.to_string(),
            Expression::Call(s) => s.to_string(),
            Expression::Array(s) => s.to_string(),
            Expression::Index(s) => s.to_string(),
            Expression::HashMapLiteral(s) => s.to_string(),
            other => other.token_literal().to_string()
        }
    }
}

