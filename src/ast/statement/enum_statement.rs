use crate::{ast::expression::Expression, token::Token};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct EnumStatement {
    pub token: Token,
    pub name: Expression,
    pub values: Vec<Expression>,
}

impl EnumStatement {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("enum ");
        buffer.push_str(&self.name.to_string());
        buffer.push_str("{ \n");

        self.values.iter().for_each(|value| {
            buffer.push('\t');
            buffer.push_str(&value.to_string());
            buffer.push('\n');
        });
        buffer.push('}');

        buffer
    }
}
