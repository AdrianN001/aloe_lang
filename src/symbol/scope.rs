use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::symbol::symbol::{Symbol, SymbolID};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct ScopeID(pub u32);

pub struct Scope {
    pub id: ScopeID,
    pub parent: Option<ScopeID>,
    pub symbols: HashMap<String, SymbolID>,
    pub local_symbol_map: HashMap<SymbolID, Symbol>,
}

impl Scope {
    pub fn new_global_scope(id: ScopeID) -> Self {
        Self {
            id,
            parent: None,
            symbols: HashMap::new(),
            local_symbol_map: HashMap::new(),
        }
    }

    pub fn new(owner_id: ScopeID, id: ScopeID) -> Self {
        Self {
            id,
            parent: Some(owner_id),
            symbols: HashMap::new(),
            local_symbol_map: HashMap::new(),
        }
    }
}
