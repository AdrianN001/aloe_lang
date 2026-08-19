use crate::{doc::doc_comment::DocComment, symbol::symbol_kind::SymbolKind};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct SymbolID(pub u64);

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Symbol {
    pub id: SymbolID,
    pub name: String,
    pub kind: SymbolKind,
    pub owner: Option<SymbolID>,
    pub doc: Option<DocComment>,
}
