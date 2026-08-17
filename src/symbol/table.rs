use std::collections::HashMap;

use crate::symbol::symbol::{Symbol, SymbolID};

pub struct SymbolTable {
    next_id: u64,
    pub symbol_map: HashMap<SymbolID, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            symbol_map: HashMap::new(),
        }
    }

    pub fn register(&mut self, symbol: Symbol) {
        self.symbol_map.insert(symbol.id, symbol);
    }

    pub fn generate_id(&mut self) -> SymbolID {
        let id = SymbolID(self.next_id);
        self.next_id += 1;
        id
    }
}
