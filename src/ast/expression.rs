pub mod array_literal;
pub mod boolean;
pub mod call_expression;
pub mod float_literal;
pub mod function_expression;
pub mod hash_map_literal;
pub mod identifier;
pub mod if_expression;
pub mod index_expression;
pub mod infix;
pub mod integer_literal;
pub mod member;
pub mod prefix_expression;
pub mod string_expr;

use array_literal::ArrayLiteral;
use boolean::Boolean;
use call_expression::CallExpression;
use float_literal::FloatLiteral;
use function_expression::FunctionExpression;
use hash_map_literal::HashMapLiteral;
use identifier::Identifier;
use if_expression::IfExpression;
use index_expression::IndexExpression;
use infix::InfixExpression;
use integer_literal::IntegerLiteral;
use member::MemberExpression;
use prefix_expression::PrefixExpression;
use string_expr::StringExpr;

#[derive(Default, PartialOrd, Ord, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    FloatLiteral(FloatLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    Bool(Boolean),
    Function(FunctionExpression),
    Call(CallExpression),
    String(StringExpr),
    Array(ArrayLiteral),
    Index(IndexExpression),
    HashMapLiteral(HashMapLiteral),

    Member(MemberExpression),

    If(IfExpression),

    #[default]
    InvalidExpression,
}

impl Expression {
    pub fn token_literal(&self) -> &str {
        match self {
            Expression::Bool(i) => &i.token.literal,
            Expression::Identifier(i) => &i.token.literal,
            Expression::IntegerLiteral(i) => &i.token.literal,
            Expression::String(str_expr) => &str_expr.value,
            _ => "",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Expression::Prefix(s) => s.to_string(),
            Expression::Infix(s) => s.to_string(),
            Expression::Call(s) => s.to_string(),
            Expression::Array(s) => s.to_string(),
            Expression::Index(s) => s.to_string(),
            Expression::HashMapLiteral(s) => s.to_string(),
            Expression::Member(s) => s.to_string(),
            Expression::FloatLiteral(s) => format!("{}.{}", s.integer_part, s.float_part),
            other => other.token_literal().to_string(),
        }
    }
}
