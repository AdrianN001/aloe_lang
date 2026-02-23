use crate::{ast::expression::Expression, token::Token};

#[derive(Default, Hash, PartialOrd, Ord, Clone, PartialEq, Eq)]
pub struct ArrayLiteral {
    pub token: Token,
    pub elements: Vec<Expression>,
}

impl ArrayLiteral {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push('[');

        buffer.push_str(
            &self
                .elements
                .iter()
                .map(|element| element.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        );
        buffer.push(']');

        buffer
    }
}
