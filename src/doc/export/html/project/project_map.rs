use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    doc::symbol::{doc_module::DocModule, doc_symbol::DocSymbol, documentation::Documentation},
    symbol::symbol_kind::SymbolKind,
};

#[derive(Serialize, Deserialize)]
pub struct SymbolEntry {
    pub name: String,
    pub doc_path: String,
    pub symbol_kind: SymbolKind,
}

/// Version mit relativem Pfad (serialisierbar)
#[derive(Serialize, Deserialize)]
pub struct SymbolEntryRelative {
    pub name: String,
    pub doc_path: String,
    pub symbol_kind: SymbolKind,
}

impl SymbolEntry {
    pub fn from_doc_symbol(symbol: &DocSymbol, module_path: &PathBuf, base_path: &PathBuf) -> Self {
        let doc_path = Self::compute_doc_path(module_path, base_path);
        SymbolEntry {
            name: symbol.name.clone(),
            doc_path,
            symbol_kind: symbol.kind.clone(),
        }
    }

    /// Konvertiert diesen Eintrag zu einem relativen Eintrag bezüglich der aktuellen Seite
    pub fn make_relative_to(
        &self,
        current_page_doc_path: &str,
        _base_path: &PathBuf,
    ) -> SymbolEntryRelative {
        let relative_doc_path =
            Self::make_path_relative_to_page(&self.doc_path, current_page_doc_path);

        SymbolEntryRelative {
            name: self.name.clone(),
            doc_path: relative_doc_path,
            symbol_kind: self.symbol_kind.clone(),
        }
    }

    /// Macht einen Pfad relativ zu einer aktuellen Seite
    /// Beide Pfade sind relativ zur base_path, z.B. "types/int.html"
    pub fn make_path_relative_to_page(target_path: &str, current_page_path: &str) -> String {
        use std::path::Component;

        let target = PathBuf::from(target_path);
        let current = PathBuf::from(current_page_path);

        let mut target_parts: Vec<_> = target.components().collect();
        let mut current_parts: Vec<_> = current.components().collect();

        // Entferne die aktuelle Datei (letztes Element)
        if !current_parts.is_empty() {
            current_parts.pop();
        }

        // Entferne gemeinsame Präfixe
        while !target_parts.is_empty()
            && !current_parts.is_empty()
            && target_parts[0] == current_parts[0]
        {
            target_parts.remove(0);
            current_parts.remove(0);
        }

        // Für jeden verbleibenden current_part, füge ".." hinzu
        let mut result = vec!["..".to_string(); current_parts.len()];

        // Füge alle target_parts hinzu
        for part in target_parts {
            if let Component::Normal(s) = part {
                result.push(s.to_string_lossy().to_string());
            }
        }

        if result.is_empty() {
            ".".to_string()
        } else {
            result.join("/")
        }
    }

    pub fn compute_doc_path(module_path: &PathBuf, base_path: &PathBuf) -> String {
        // Absolutiere beide Pfade, damit sie vergleichbar sind
        let abs_module_path = if module_path.is_absolute() {
            module_path.clone()
        } else {
            std::env::current_dir()
                .unwrap_or_default()
                .join(module_path)
        };

        let abs_base_path = if base_path.is_absolute() {
            base_path.clone()
        } else {
            std::env::current_dir().unwrap_or_default().join(base_path)
        };

        // Berechne den relativen Pfad vom Basis-Verzeichnis zum Modul
        let rel_path = abs_module_path
            .strip_prefix(&abs_base_path)
            .unwrap_or(&abs_module_path);

        // Ändere Dateiendung auf .html
        let html_path = rel_path.with_extension("html");

        // Konvertiere zu String mit forward slashes (URL-Format)
        let path_str = html_path.to_string_lossy().replace("\\", "/");

        path_str
    }
}

pub struct DocProjectMap<'a> {
    pub module_map: HashMap<PathBuf, &'a DocModule>,
    pub symbol_vec: Vec<SymbolEntry>,
    pub base_path: PathBuf,
}

impl<'a> DocProjectMap<'a> {
    pub fn new_from_documentation(doc: &'a Documentation, base_path: PathBuf) -> Self {
        let map = Self::build_module_map(&doc.modules);
        let symbol_vec = Self::build_symbol_vec(doc, &base_path);
        Self {
            module_map: map,
            symbol_vec: symbol_vec,
            base_path,
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

    fn build_symbol_vec(doc: &'a Documentation, base_path: &PathBuf) -> Vec<SymbolEntry> {
        let mut vec = Vec::new();
        for module in &doc.modules {
            if let Some(module_path) = &module.path {
                for symbol in &module.all_symbols {
                    vec.push(SymbolEntry::from_doc_symbol(symbol, module_path, base_path));
                }
            }
        }
        vec.sort_by_key(|symbol_entry| symbol_entry.name.clone());
        vec
    }

    /// Berechnet die doc_path Werte relativ zur gegebenen Seite
    /// current_page_doc_path sollte ein relativer HTML-Pfad vom base_path sein, z.B. "types/int.html"
    pub fn get_symbol_entries_relative_to(
        &self,
        current_page_doc_path: &str,
    ) -> Vec<SymbolEntryRelative> {
        self.symbol_vec
            .iter()
            .map(|entry| entry.make_relative_to(current_page_doc_path, &self.base_path))
            .collect()
    }
}
