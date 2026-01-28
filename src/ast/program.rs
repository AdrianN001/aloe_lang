
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
}

