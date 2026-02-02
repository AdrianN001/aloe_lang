
use crate::ast::statement::Statement;

#[derive(Default)]
pub struct Program{
    pub statements: Vec<Statement>
}

impl Program{
    pub fn new() -> Self{
        Program::default()
    }
    fn token_literal(&self) -> Option<&str>{
        if !self.statements.is_empty() {
            return Some(self.statements[0].token_literal());
        }
        None
    }

    pub fn to_string(&self) -> String{
        let mut buffer = String::new();

        self.statements.iter().for_each(|statement|{
            buffer.push_str(&statement.to_string());
            //buffer.push('\n');
        });

        buffer
    }
}

