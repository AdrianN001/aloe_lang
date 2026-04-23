use crate::{
    ast::{expression::identifier::Identifier, statement::block_statement::BlockStatement},
    token::Token,
};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Default, Eq, Debug)]
pub struct FunctionStatement {
    pub token: Token,
    pub name: String,
    pub parameters: Vec<Identifier>,
    pub block: BlockStatement,
}

impl FunctionStatement {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str(&self.token.literal);
        buffer.push_str(&self.name);
        buffer.push('(');
        buffer.push_str(
            &self
                .parameters
                .iter()
                .map(|parameter| parameter.token.literal.clone())
                .collect::<Vec<_>>()
                .join(", "),
        );
        buffer.push(')');

        buffer.push_str(&self.block.to_string());

        buffer
    }
}
