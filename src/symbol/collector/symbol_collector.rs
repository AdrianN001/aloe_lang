use std::{collections::HashMap, unreachable};

use crate::{
    ast::{
        expression::Expression, program::Program, statement::Statement,
        syntax_error_report::syntax_error::SyntaxError,
    },
    symbol::{
        scope::{Scope, ScopeID},
        symbol::{Symbol, SymbolID},
        table::SymbolTable,
    },
};

pub struct SymbolCollector {
    pub table: SymbolTable,
    pub current_scope_id: ScopeID,
}

impl SymbolCollector {
    fn new() -> Self {
        let global_scope_id = SymbolTable::GLOBAL_SCOPE_ID;
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
            Statement::AsyncFunction(async_function_stmt) => {
                self.handle_async_function(async_function_stmt)
            }
            Statement::Struct(struct_model_stmt) => self.handle_struct_statement(struct_model_stmt),

            Statement::Expression(expression_stmt) => {
                self.collect_expression(&expression_stmt.expression)
            }
            _ => Ok(()),
        }
    }

    pub fn collect_expression(&mut self, expression: &Expression) -> Result<(), SyntaxError> {
        match expression {
            Expression::If(if_expr) => self.handle_if_expression(&if_expr),
            Expression::WhileLoop(while_loop_expr) => {
                self.handle_while_expression(&while_loop_expr)
            }
            Expression::ForLoop(for_loop_expr) => self.handle_for_expression(&for_loop_expr),
            _ => Ok(()),
        }
    }

    pub fn get_top_level_symbol_map<'a>(&'a self) -> &'a HashMap<SymbolID, Symbol> {
        let global_scope_id = SymbolTable::GLOBAL_SCOPE_ID;
        let global_scope = match self.table.scopes.get(&global_scope_id) {
            Some(scope) => scope,
            None => unreachable!(),
        };

        &global_scope.local_symbol_map
    }

    pub fn get_global_symbol_map<'a>(&'a self) -> &'a HashMap<SymbolID, Symbol> {
        &self.table.symbol_map
    }
}
