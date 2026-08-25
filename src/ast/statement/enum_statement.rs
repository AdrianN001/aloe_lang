use crate::{
    ast::expression::Expression,
    doc::{doc_comment::DocComment, traits::statement_traits::Documentable},
    token::Token,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct EnumStatement {
    pub token: Token,
    pub name: Expression,
    pub values: Vec<Expression>,
    pub doc_comment: Option<DocComment>,
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

impl Documentable for EnumStatement {
    fn set_doc_comment(&mut self, doc_comment: Token) {
        self.doc_comment = Some(DocComment::new_from_token(doc_comment));
    }
}
