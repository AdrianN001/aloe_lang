use std::panic;

use crate::{
    ast::{program::Program, statement::Statement, syntax_error_report::syntax_error::SyntaxError},
    symbol::table::SymbolTable,
};

pub struct SymbolCollector {
    pub table: SymbolTable,
}

impl SymbolCollector {
    fn new() -> Self {
        Self {
            table: SymbolTable::new(),
        }
    }
    pub fn collect_from_program(program: &Program) -> Result<Self, SyntaxError> {
        let mut collector = SymbolCollector::new();

        for statement in &program.statements {
            collector.collect_statement(statement)?;
        }

        Ok(collector)
    }

    fn collect_statement(&mut self, statement: &Statement) -> Result<(), SyntaxError> {
        match statement {
            Statement::Let(let_stmt) => self.handle_let_statement(let_stmt),
            Statement::Val(val_stmt) => self.handle_val_statement(val_stmt),
            Statement::Import(import_stmt) => self.handle_import_statement(import_stmt),
            Statement::Enum(enum_stmt) => self.handle_enum_statement(enum_stmt),
            _ => panic!(),
        }
    }
}
