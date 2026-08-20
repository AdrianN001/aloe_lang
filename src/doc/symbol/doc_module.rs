use std::{collections::HashMap, unreachable};

use serde::{Deserialize, Serialize};

use crate::{
    doc::symbol::doc_symbol::DocSymbol,
    symbol::{
        collector::symbol_collector::SymbolCollector,
        symbol::{Symbol, SymbolID},
        table::SymbolTable,
    },
};

#[derive(Serialize, Deserialize)]
pub struct DocModule {
    pub name: String,
    pub root_symbols: Vec<DocSymbol>,
}

type OwnershipMap = HashMap<Option<SymbolID>, Vec<SymbolID>>;
type SymbolMap = HashMap<SymbolID, Symbol>;

impl DocModule {
    pub fn from_symbol_collector(name: &str, collector: &SymbolCollector) -> DocModule {
        let symbol_map = collector.get_global_symbol_map();

        let ownership_map = DocModule::build_ownership_map(symbol_map);
        let root_symbols = DocModule::make_root_symbols(&ownership_map, symbol_map)
            .iter()
            .filter(|symbol| symbol.scope_id == SymbolTable::GLOBAL_SCOPE_ID)
            .map(|symbol| symbol.clone())
            .collect::<Vec<DocSymbol>>();

        DocModule {
            name: name.to_string(),
            root_symbols,
        }
    }

    fn build_ownership_map(symbol_map: &HashMap<SymbolID, Symbol>) -> OwnershipMap {
        let mut ownership: OwnershipMap = HashMap::new();

        for symbol in symbol_map.values() {
            let owner_of_symbol = &symbol.owner;

            match ownership.get_mut(owner_of_symbol) {
                Some(children) => children.push(symbol.id),
                None => {
                    ownership.insert(*owner_of_symbol, vec![symbol.id]);
                }
            };
        }

        ownership
    }

    fn make_root_symbols(ownership_map: &OwnershipMap, symbol_map: &SymbolMap) -> Vec<DocSymbol> {
        // top-level symbols
        let top_level_symbol_ids = match ownership_map.get(&None) {
            Some(top_level_symbol_ids) => top_level_symbol_ids,
            None => unreachable!(), // es gibt immer a symbol mit keinem "owner"
        };

        let mut unsorted_symbols: Vec<DocSymbol> = top_level_symbol_ids
            .iter()
            .map(|top_level_id| {
                DocModule::build_doc_symbol(*top_level_id, ownership_map, symbol_map)
            })
            .collect();

        unsorted_symbols.sort_by_key(|doc_symbol| doc_symbol.id);

        unsorted_symbols
    }

    fn build_doc_symbol(
        id: SymbolID,
        ownership_map: &OwnershipMap,
        symbol_map: &SymbolMap,
    ) -> DocSymbol {
        let symbol = match symbol_map.get(&id) {
            Some(symbol) => symbol,
            None => unreachable!(),
        };

        let mut docsymbol = DocSymbol::new_from_symbol(symbol);

        let children = match ownership_map.get(&Some(id)) {
            None => return docsymbol,
            Some(children) => children,
        };

        for child_id in children {
            let child_doc_symbol =
                DocModule::build_doc_symbol(*child_id, ownership_map, symbol_map);
            docsymbol.add_child(child_doc_symbol);
        }

        //sort the child
        docsymbol.children.sort_by_key(|doc_symbol| doc_symbol.id);

        docsymbol
    }
}
