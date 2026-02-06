use crate::token::Token;



#[derive(Clone, Default, PartialEq, Eq)]
pub struct Boolean{
    pub token: Token,
    pub value: bool
}


