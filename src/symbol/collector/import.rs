use crate::{
    ast::{
        statement::import_statement::ImportStatement,
        syntax_error_report::syntax_error::SyntaxError,
    },
    symbol::{
        collector::symbol_collector::SymbolCollector, symbol::Symbol, symbol_kind::SymbolKind,
    },
};

impl SymbolCollector {
    pub fn handle_import_statement(
        &mut self,
        statement: &ImportStatement,
    ) -> Result<(), SyntaxError> {
        if let Some(module_name) = &statement.custom_name {
            self.register_new_module(module_name);
        }
        self.add_import(&statement.module_name);
        Ok(())
    }

    fn register_new_module(&mut self, name: &str) {
        let new_symbol_id = self.table.generate_symbol_id();
        let new_symbol = Symbol {
            id: new_symbol_id,

            scope_id: self.current_scope_id,
            name: name.to_string(),
            kind: SymbolKind::Module,
            owner: None,
            doc: None,
        };

        self.table.register_to_symbolmap(new_symbol.clone());
        self.table
            .register_to_scope(self.current_scope_id, new_symbol);
    }

    fn add_import(&mut self, path: &str) {
        if self.should_add_import(path) {
            self.imports.insert(path.to_string());
        }
    }

    fn should_add_import(&self, path: &str) -> bool {
        !path.starts_with("@std")
    }
}
