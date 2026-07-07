use crate::token::Token;

pub trait Documentable {
    fn set_doc_comment(&mut self, doc_comment: Token);
}
