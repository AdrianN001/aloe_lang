use crate::{ast::statement::Statement, token::Token};


#[derive(Default, Clone)]
pub struct BlockStatement{
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl BlockStatement {
    pub fn to_string(&self) -> String{
        let mut buffer = String::new();

        self.statements.iter().for_each(|statement|{
            buffer.push_str(&statement.to_string());
        });

        buffer
    }
}
