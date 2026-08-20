use std::{collections::HashMap, unreachable};

use crate::symbol::{
    scope::{Scope, ScopeID},
    symbol::{Symbol, SymbolID},
};

pub struct SymbolTable {
    next_symbol_id: u64,
    next_scope_id: u32,
    pub symbol_map: HashMap<SymbolID, Symbol>,
    pub scopes: HashMap<ScopeID, Scope>,
}

impl SymbolTable {
    pub const TOP_LEVEL_SCOPE_ID: ScopeID = ScopeID(1);

    pub fn new() -> Self {
        Self {
            next_symbol_id: 1,
            next_scope_id: 2,
            symbol_map: HashMap::new(),
            scopes: HashMap::new(),
        }
    }

    pub fn register_to_symbolmap(&mut self, symbol: Symbol) {
        self.symbol_map.insert(symbol.id, symbol);
    }

    pub fn register_to_scope(&mut self, current_scope_id: ScopeID, symbol: Symbol) {
        let current_scope = match self.scopes.get_mut(&current_scope_id) {
            Some(current_scope) => current_scope,
            None => unreachable!(),
        };

        current_scope
            .local_symbol_map
            .insert(symbol.id, symbol.clone());
        current_scope.symbols.insert(symbol.name, symbol.id);
    }

    pub fn register_new_scope(&mut self, scope: Scope) {
        self.scopes.insert(scope.id, scope);
    }

    pub fn generate_symbol_id(&mut self) -> SymbolID {
        let id = SymbolID(self.next_symbol_id);
        self.next_symbol_id += 1;
        id
    }

    pub fn generate_scope_id(&mut self) -> ScopeID {
        let id = ScopeID(self.next_scope_id);
        self.next_scope_id += 1;
        id
    }

    pub fn get_symbol(&mut self, symbol_id: SymbolID) -> Option<&Symbol> {
        self.symbol_map.get(&symbol_id)
    }
}
