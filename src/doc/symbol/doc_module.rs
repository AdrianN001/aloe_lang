use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    unreachable,
};

use serde::{Deserialize, Serialize};

use crate::{
    ast::{Parser, syntax_error_report::syntax_error::SyntaxError},
    doc::symbol::doc_symbol::DocSymbol,
    lexer::Lexer,
    symbol::{
        collector::symbol_collector::SymbolCollector,
        symbol::{Symbol, SymbolID},
        symbol_kind::SymbolKind,
        table::SymbolTable,
    },
};

#[derive(Serialize, Deserialize)]
pub struct DocModule {
    pub name: String,
    pub path: Option<PathBuf>,
    pub root_symbols: Vec<DocSymbol>,

    pub imports: HashSet<String>,
    pub all_symbols: Vec<DocSymbol>,
}

type OwnershipMap = HashMap<Option<SymbolID>, Vec<SymbolID>>;
type SymbolMap = HashMap<SymbolID, Symbol>;

impl DocModule {
    pub fn from_symbol_collector(name: &str, collector: &SymbolCollector) -> DocModule {
        let symbol_map = collector.get_global_symbol_map();
        let all_symbols = {
            collector
                .table
                .symbol_map
                .values()
                .filter(|symbol| symbol.scope_id == SymbolTable::TOP_LEVEL_SCOPE_ID)
                .map(|symbol| DocSymbol::new_from_symbol(symbol))
                .collect::<Vec<DocSymbol>>()
        };

        let ownership_map = DocModule::build_ownership_map(symbol_map);
        let root_symbols = DocModule::make_root_symbols(&ownership_map, symbol_map)
            .iter()
            .filter(|symbol| symbol.scope_id == SymbolTable::TOP_LEVEL_SCOPE_ID)
            .map(|symbol| symbol.clone())
            .collect::<Vec<DocSymbol>>();

        DocModule {
            name: name.to_string(),
            root_symbols,
            imports: collector.imports.iter().cloned().collect(),
            all_symbols,
            path: None,
        }
    }

    pub fn from_single_input(name: &str, input: &str) -> Result<DocModule, SyntaxError> {
        let lexer = Lexer::new(input.to_string());
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let collector = SymbolCollector::collect_from_program(&program)?;

        Ok(DocModule::from_symbol_collector(name, &collector))
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
            None => &vec![], // es gibt immer a symbol mit keinem "owner"
        };

        let mut unsorted_symbols: Vec<DocSymbol> = top_level_symbol_ids
            .iter()
            .map(|id| DocModule::build_doc_symbol(*id, ownership_map, symbol_map))
            .collect();

        DocModule::purge_non_documented_symbols(&mut unsorted_symbols);
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

    fn purge_non_documented_symbols(symbols: &mut Vec<DocSymbol>) {
        symbols.retain(|symbol| !Self::should_symbol_be_purged(symbol));
    }

    fn should_symbol_be_purged(symbol: &DocSymbol) -> bool {
        symbol.doc.is_none()
            && matches!(
                symbol.kind,
                SymbolKind::Struct
                    | SymbolKind::Enum
                    | SymbolKind::AsyncFunction
                    | SymbolKind::Function
                    | SymbolKind::StructAsyncMethod
                    | SymbolKind::StructMethod
            )
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = Some(path);
    }
}
