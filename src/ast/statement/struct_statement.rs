use crate::{ast::{expression::Expression, statement::Statement}, token::Token};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct StructStatement {
    pub token: Token,
    pub name: Expression,
    pub attributes: Vec<Expression>,
    pub methods: Vec<Statement>
}

impl StructStatement {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("struct ");
        buffer.push_str(&self.name.to_string());
        buffer.push_str("{ \n");

        self.attributes.iter().for_each(|attribute| {
            buffer.push('\t');
            buffer.push_str(&attribute.to_string());
            buffer.push('\n');
        });
        buffer.push('}');
        buffer.push(';');

        buffer
    }
}
