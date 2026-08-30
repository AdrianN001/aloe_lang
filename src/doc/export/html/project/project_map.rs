use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    doc::symbol::{doc_module::DocModule, doc_symbol::DocSymbol, documentation::Documentation},
    symbol::symbol_kind::SymbolKind,
};

#[derive(Serialize, Deserialize)]
pub struct SymbolEntry {
    name: String,
    path: PathBuf,
    symbol_kind: SymbolKind,
}

impl SymbolEntry {
    pub fn from_doc_symbol(symbol: &DocSymbol, path: PathBuf) -> Self {
        SymbolEntry {
            name: symbol.name.clone(),
            path: path,
            symbol_kind: symbol.kind.clone(),
        }
    }
}

pub struct DocProjectMap<'a> {
    pub module_map: HashMap<PathBuf, &'a DocModule>,
    pub symbol_vec: Vec<SymbolEntry>,
}

impl<'a> DocProjectMap<'a> {
    pub fn new_from_documentation(doc: &'a Documentation) -> Self {
        let map = Self::build_module_map(&doc.modules);
        let symbol_vec = Self::build_symbol_vec(doc);
        Self {
            module_map: map,
            symbol_vec: symbol_vec,
        }
    }

    fn build_module_map(doc: &'a Vec<DocModule>) -> HashMap<PathBuf, &'a DocModule> {
        let mut map = HashMap::new();
        for module in doc {
            if let Some(module_path) = module.path.clone() {
                map.insert(module_path.clone(), module);
            }
        }
        map
    }

    fn build_symbol_vec(doc: &'a Documentation) -> Vec<SymbolEntry> {
        let mut vec = Vec::new();
        for module in &doc.modules {
            if let Some(module_path) = &module.path {
                for symbol in &module.all_symbols {
                    vec.push(SymbolEntry::from_doc_symbol(symbol, module_path.clone()));
                }
            }
        }
        vec.sort_by_key(|symbol_entry| symbol_entry.name.clone());
        vec
    }
}
