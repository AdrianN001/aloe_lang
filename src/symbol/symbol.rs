use serde::{Deserialize, Serialize};

use crate::{
    doc::doc_comment::DocComment,
    symbol::{scope::ScopeID, symbol_kind::SymbolKind},
};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SymbolID(pub u64);

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Symbol {
    pub id: SymbolID,
    pub scope_id: ScopeID,
    pub name: String,
    pub kind: SymbolKind,
    pub owner: Option<SymbolID>,
    pub doc: Option<DocComment>,
}
