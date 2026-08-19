use crate::symbol::{collector::symbol_collector::SymbolCollector, scope::Scope};

impl SymbolCollector {
    pub fn new_scope(&mut self) {
        let old_scope_id = &self.current_scope_id;

        let new_scope_id = self.table.generate_scope_id();
        let new_scope = Scope::new(old_scope_id.clone(), new_scope_id);

        self.table.register_new_scope(new_scope);

        self.current_scope_id = new_scope_id
    }

    pub fn previous_scope(&mut self) -> Result<(), Box<&str>> {
        let current_scope_id = self.current_scope_id;

        let current_scope = match self.table.scopes.get(&current_scope_id) {
            Some(current_scope) => current_scope,
            None => return Err(Box::new("invalid current scope_id")),
        };

        let prev_scope_id = match current_scope.parent {
            Some(prev_scope_id) => prev_scope_id,
            None => return Err(Box::new("scope has no parent")),
        };

        self.current_scope_id = prev_scope_id;

        Ok(())
    }
}
