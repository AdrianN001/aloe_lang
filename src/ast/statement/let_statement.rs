use crate::ast::expression::Expression;
use crate::doc::doc_comment::DocComment;
use crate::doc::traits::statement_traits::Documentable;
use crate::token::Token;

#[derive(Default, Hash, PartialOrd, Ord, Clone, PartialEq, Eq, Debug)]
pub struct LetStatement {
    pub token: Token,
    pub assignment: Expression,
    pub doc_comment: Option<DocComment>,
}

impl LetStatement {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("let ");

        buffer.push_str(&self.assignment.to_string());

        buffer.push(';');
        buffer
    }
}

impl Documentable for LetStatement {
    fn set_doc_comment(&mut self, doc_comment: Token) {
        self.doc_comment = Some(DocComment::new_from_token(doc_comment));
    }
}
