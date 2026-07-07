use crate::{
    ast::{expression::Expression, statement::Statement},
    doc::{doc_comment::DocComment, traits::statement_traits::Documentable},
    token::Token,
};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct StructStatement {
    pub token: Token,
    pub name: Expression,
    pub attributes: Vec<Expression>,
    pub methods: Vec<Statement>,
    pub doc_comment: Option<DocComment>,
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

        buffer
    }
}

impl Documentable for StructStatement {
    fn set_doc_comment(&mut self, doc_comment: Token) {
        self.doc_comment = Some(DocComment::new_from_token(doc_comment));
    }
}
