use std::collections::HashMap;

use crate::symbol::symbol::SymbolID;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct ScopeID(pub u32);

pub struct Scope {
    pub id: ScopeID,
    pub parent: Option<ScopeID>,
    pub symbols: HashMap<String, SymbolID>,
}

impl Scope {
    pub fn new_global_scope(id: ScopeID) -> Self {
        Self {
            id,
            parent: None,
            symbols: HashMap::new(),
        }
    }

    pub fn new(owner_id: ScopeID, id: ScopeID) -> Self {
        Self {
            id,
            parent: Some(owner_id),
            symbols: HashMap::new(),
        }
    }
}
