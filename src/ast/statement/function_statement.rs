use crate::{
    ast::{expression::identifier::Identifier, statement::block_statement::BlockStatement},
    doc::{doc_comment::DocComment, traits::statement_traits::Documentable},
    token::Token,
};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Default, Eq, Debug)]
pub struct FunctionStatement {
    pub token: Token,
    pub name: String,
    pub parameters: Vec<Identifier>,
    pub block: BlockStatement,
    pub doc_comment: Option<DocComment>,
}

impl FunctionStatement {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str(&self.token.literal);
        buffer.push(' ');
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

impl Documentable for FunctionStatement {
    fn set_doc_comment(&mut self, doc_comment: Token) {
        self.doc_comment = Some(DocComment::new_from_token(doc_comment));
    }
}
