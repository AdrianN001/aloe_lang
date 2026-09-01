use crate::doc::export::html::project::project_map::{SymbolEntry, SymbolEntryRelative};
use crate::symbol::symbol_kind::SymbolKind;
use std::path::PathBuf;

#[test]
fn test_compute_doc_path_simple() {
    // Test: Modul im Root-Verzeichnis
    let base_path = PathBuf::from("/home/user/proj");
    let module_path = PathBuf::from("/home/user/proj/main.aloe");

    let expected = "main.html";
    let result = SymbolEntry::compute_doc_path(&module_path, &base_path);

    assert_eq!(result, expected);
}

#[test]
fn test_compute_doc_path_nested() {
    // Test: Modul in verschachteltem Verzeichnis
    let base_path = PathBuf::from("/home/user/proj");
    let module_path = PathBuf::from("/home/user/proj/lib/utils/math.aloe");

    let expected = "lib/utils/math.html";
    let result = SymbolEntry::compute_doc_path(&module_path, &base_path);

    assert_eq!(result, expected);
}

#[test]
fn test_compute_doc_path_relative_inputs() {
    // Test: Relative Eingabepfade werden absolutiert
    let base_path = PathBuf::from("./docs");
    let module_path = PathBuf::from("./docs/types/int.aloe");

    let result = SymbolEntry::compute_doc_path(&module_path, &base_path);

    assert_eq!(result, "types/int.html");
}

#[test]
fn test_make_path_relative_same_directory() {
    // Test: Pfade im gleichen Verzeichnis
    // target im Root, current im "types/" Verzeichnis → brauchen wir hochgehen
    let target = "int.html";
    let current = "types/array.html";

    let result = SymbolEntry::make_path_relative_to_page(target, current);

    // current ist in "types/", target ist im Root
    // Wir brauchen: ../int.html
    assert_eq!(result, "../int.html");
}

#[test]
fn test_make_path_relative_different_depths() {
    // Test: target im Root, current tiefer verschachtelt
    let target = "config.html";
    let current = "types/utils/helpers.html";

    let result = SymbolEntry::make_path_relative_to_page(target, current);

    // Wir sind in types/utils/ und wollen zu Root/config.html
    // Wir müssen 2x ".." gehen: "../../config.html"
    assert_eq!(result, "../../config.html");
}

#[test]
fn test_make_path_relative_target_deeper() {
    // Test: target tiefer verschachtelt als current
    let target = "types/methods/string.html";
    let current = "main.html";

    let result = SymbolEntry::make_path_relative_to_page(target, current);

    // Wir sind in Root und wollen zu types/methods/string.html
    assert_eq!(result, "types/methods/string.html");
}

#[test]
fn test_make_path_relative_both_nested() {
    // Test: Beide Pfade sind verschachtelt, aber in unterschiedlichen Ästen
    let target = "core/math.html";
    let current = "utils/helpers.html";

    let result = SymbolEntry::make_path_relative_to_page(target, current);

    // Von utils/ zu core/: "../core/math.html"
    assert_eq!(result, "../core/math.html");
}

#[test]
fn test_symbol_entry_relative_creation() {
    // Test: Erstelle einen SymbolEntry und konvertiere ihn zu relativ
    let entry = SymbolEntry {
        name: "StringType".to_string(),
        doc_path: "types/str.html".to_string(),
        symbol_kind: SymbolKind::Struct,
    };

    let base_path = PathBuf::from("/home/user/proj/docs");
    let current_page = "docs/std.html";

    let relative = entry.make_relative_to(current_page, &base_path);

    // Verifiziere dass die konvertierte Version alle Felder hat
    assert_eq!(relative.name, "StringType");
    assert_eq!(relative.symbol_kind, SymbolKind::Struct);
    // Von docs/ zu types/str.html brauchen wir: ../types/str.html
    assert_eq!(relative.doc_path, "../types/str.html");
}

#[test]
fn test_symbol_entry_relative_same_level() {
    // Test: Current und target im gleichen Verzeichnis
    let entry = SymbolEntry {
        name: "ArrayType".to_string(),
        doc_path: "types/array.html".to_string(),
        symbol_kind: SymbolKind::Struct,
    };

    let base_path = PathBuf::from("/home/user/proj");
    let current_page = "types/int.html";

    let relative = entry.make_relative_to(current_page, &base_path);

    // Wenn beide im "types/" sind, sollte der Pfad einfach "array.html" sein
    assert_eq!(relative.doc_path, "array.html");
}

#[test]
fn test_compute_doc_path_windows_style() {
    // Test: Windows-Pfade werden korrekt konvertiert
    #[cfg(target_os = "windows")]
    {
        let base_path = PathBuf::from("C:\\Users\\user\\proj");
        let module_path = PathBuf::from("C:\\Users\\user\\proj\\lib\\utils.aloe");

        let result = SymbolEntry::compute_doc_path(&module_path, &base_path);

        // Sollte forward slashes verwenden
        assert!(!result.contains("\\"));
        assert_eq!(result, "lib/utils.html");
    }
}

#[test]
fn test_make_path_relative_three_levels_deep() {
    // Test: Complex nesting scenario
    let target = "modules/core/types/int.html";
    let current = "docs/api/reference.html";

    let result = SymbolEntry::make_path_relative_to_page(target, current);

    // Von docs/api/ zu modules/core/types/int.html
    // Wir brauchen: ../../modules/core/types/int.html
    assert_eq!(result, "../../modules/core/types/int.html");
}

#[test]
fn test_symbol_entry_from_doc_symbol() {
    // Test: SymbolEntry.compute_doc_path berechnet korrekten doc_path
    let module_path = PathBuf::from("/home/user/proj/lib/types.aloe");
    let base_path = PathBuf::from("/home/user/proj");

    let doc_path = SymbolEntry::compute_doc_path(&module_path, &base_path);

    assert_eq!(doc_path, "lib/types.html");

    // Erstelle einen SymbolEntry manuell
    let entry = SymbolEntry {
        name: "MyStruct".to_string(),
        doc_path,
        symbol_kind: SymbolKind::Struct,
    };

    assert_eq!(entry.name, "MyStruct");
    assert_eq!(entry.doc_path, "lib/types.html");
    assert_eq!(entry.symbol_kind, SymbolKind::Struct);
}
