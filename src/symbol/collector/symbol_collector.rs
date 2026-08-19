use std::panic;

use crate::{
    ast::{program::Program, statement::Statement, syntax_error_report::syntax_error::SyntaxError},
    symbol::{
        scope::{Scope, ScopeID},
        table::SymbolTable,
    },
};

pub struct SymbolCollector {
    pub table: SymbolTable,
    pub current_scope_id: ScopeID,
}

impl SymbolCollector {
    fn new() -> Self {
        let global_scope_id = ScopeID(1);
        let global_scope = Scope::new_global_scope(global_scope_id);

        let mut new_collector = Self {
            table: SymbolTable::new(),
            current_scope_id: global_scope_id,
        };

        new_collector.table.register_new_scope(global_scope);

        new_collector
    }
    pub fn collect_from_program(program: &Program) -> Result<Self, SyntaxError> {
        let mut collector = SymbolCollector::new();

        for statement in &program.statements {
            collector.collect_statement(statement)?;
        }

        Ok(collector)
    }

    pub fn collect_statement(&mut self, statement: &Statement) -> Result<(), SyntaxError> {
        match statement {
            Statement::Let(let_stmt) => self.handle_let_statement(let_stmt),
            Statement::Val(val_stmt) => self.handle_val_statement(val_stmt),
            Statement::Import(import_stmt) => self.handle_import_statement(import_stmt),
            Statement::Enum(enum_stmt) => self.handle_enum_statement(enum_stmt),
            Statement::Function(function_stmt) => self.handle_function_statement(function_stmt),
            _ => panic!(),
        }
    }
}
