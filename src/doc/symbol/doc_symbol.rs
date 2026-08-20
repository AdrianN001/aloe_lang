use crate::{
    doc::doc_comment::DocComment,
    symbol::{
        scope::ScopeID,
        symbol::{Symbol, SymbolID},
        symbol_kind::SymbolKind,
    },
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DocSymbol {
    pub id: SymbolID,
    pub scope_id: ScopeID,
    pub name: String,
    pub kind: SymbolKind,
    pub doc: Option<DocComment>,
    pub children: Vec<DocSymbol>,
}

impl DocSymbol {
    pub fn new_from_symbol(symbol: &Symbol) -> Self {
        Self {
            id: symbol.id,
            scope_id: symbol.scope_id.clone(),
            name: symbol.name.clone(),
            kind: symbol.kind,
            doc: symbol.doc.clone(),
            children: vec![],
        }
    }

    pub fn has_doc(&self) -> bool {
        self.doc.is_some()
    }

    pub fn add_child(&mut self, child: Self) {
        self.children.push(child);
    }
}
