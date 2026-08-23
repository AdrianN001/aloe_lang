use crate::doc::doc_comment::parsed::ParsedDocComment;
use crate::{
    doc::doc_comment::DocComment,
    symbol::{
        scope::ScopeID,
        symbol::{Symbol, SymbolID},
        symbol_kind::SymbolKind,
    },
};

use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Template)]
#[template(path = "symbol.html")]
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

    pub fn get_name(&self) -> &str {
        if self.kind == SymbolKind::Function
            || self.kind == SymbolKind::AsyncFunction
            || self.kind == SymbolKind::StructMethod
            || self.kind == SymbolKind::StructAsyncMethod
        {
            if let Some(doc) = &self.doc {
                if let Some(parsed) = &doc.parsed_content {
                    if let ParsedDocComment::Function { declaration, .. } = parsed {
                        return declaration;
                    }
                }
            }
            &self.name
        } else {
            &self.name
        }
    }
}
