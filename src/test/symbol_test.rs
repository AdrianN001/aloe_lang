use std::assert_eq;

use crate::{
    ast::Parser,
    lexer::Lexer,
    symbol::{
        collector::symbol_collector::SymbolCollector,
        symbol::SymbolID,
        symbol_kind::SymbolKind::{self, LetVariable, ValVariable},
    },
};

#[test]
pub fn test_let_statement_symbols() {
    let testcases = [
        (
            "let variable = \"abc\";",
            vec![("variable", 1, LetVariable, None)],
        ),
        (
            r#"
##
# variable documentation
##
let variable = 5;
"#,
            vec![("variable", 1, LetVariable, Some("variable documentation"))],
        ),
        (
            "let [variable_a, variable_b] = [0,1];",
            vec![
                ("variable_a", 1, LetVariable, None),
                ("variable_b", 2, LetVariable, None),
            ],
        ),
        (
            r#"
##
# variable_a doc, variable_b doc
##          
let [variable_a, variable_b] = [0,1];
"#,
            vec![
                (
                    "variable_a",
                    1,
                    LetVariable,
                    Some("variable_a doc, variable_b doc"),
                ),
                (
                    "variable_b",
                    2,
                    LetVariable,
                    Some("variable_a doc, variable_b doc"),
                ),
            ],
        ),
        (
            "
        let variable_a = 0;
        let [variable_b, variable_c] = [1,2];
        ",
            vec![
                ("variable_a", 1, LetVariable, None),
                ("variable_b", 2, LetVariable, None),
                ("variable_c", 3, LetVariable, None),
            ],
        ),
        (
            "
##
# variable_a doc
##
let variable_a = 0;

##
# variable_b doc, variable_c doc
##
let [variable_b, variable_c] = [1,2];
        ",
            vec![
                ("variable_a", 1, LetVariable, Some("variable_a doc")),
                (
                    "variable_b",
                    2,
                    LetVariable,
                    Some("variable_b doc, variable_c doc"),
                ),
                (
                    "variable_c",
                    3,
                    LetVariable,
                    Some("variable_b doc, variable_c doc"),
                ),
            ],
        ),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0;
        let expected_symbols: &Vec<(&str, usize, SymbolKind, Option<&str>)> = &testcase.1;

        let lexer = Lexer::new(input.to_string());
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let collector = match SymbolCollector::collect_from_program(&program) {
            Ok(collector) => collector,
            Err(_) => {
                assert!(false);
                return;
            }
        };

        assert_eq!(collector.table.symbol_map.len(), expected_symbols.len());

        expected_symbols.iter().for_each(|expected| {
            let expected_name = expected.0;
            let expected_symbol_id = SymbolID(expected.1 as u64);
            let expected_symbol_kind = &expected.2;
            let expected_doc = expected.3;

            let symbol = match collector.table.symbol_map.get(&expected_symbol_id) {
                Some(symbol) => symbol,
                None => {
                    assert!(false);
                    return;
                }
            };

            assert_eq!(expected_doc.is_some(), symbol.doc.is_some());

            if let Some(doc) = expected_doc
                && let Some(symbol_doc) = &symbol.doc
            {
                assert_eq!(doc, symbol_doc.raw_content);
            }

            assert_eq!(expected_name, symbol.name);
            assert_eq!(expected_symbol_id, symbol.id);
            assert_eq!(*expected_symbol_kind, symbol.kind);
        });
    });
}
#[test]
pub fn test_val_statement_symbols() {
    let testcases = [
        (
            "val variable = \"abc\";",
            vec![("variable", 1, ValVariable, None)],
        ),
        (
            r#"
##
# variable documentation
##
val variable = 5;
"#,
            vec![("variable", 1, ValVariable, Some("variable documentation"))],
        ),
        (
            "val [variable_a, variable_b] = [0,1];",
            vec![
                ("variable_a", 1, ValVariable, None),
                ("variable_b", 2, ValVariable, None),
            ],
        ),
        (
            r#"
##
# variable_a doc, variable_b doc
##          
val [variable_a, variable_b] = [0,1];
"#,
            vec![
                (
                    "variable_a",
                    1,
                    ValVariable,
                    Some("variable_a doc, variable_b doc"),
                ),
                (
                    "variable_b",
                    2,
                    ValVariable,
                    Some("variable_a doc, variable_b doc"),
                ),
            ],
        ),
        (
            "
        val variable_a = 0;
        val [variable_b, variable_c] = [1,2];
        ",
            vec![
                ("variable_a", 1, ValVariable, None),
                ("variable_b", 2, ValVariable, None),
                ("variable_c", 3, ValVariable, None),
            ],
        ),
        (
            "
##
# variable_a doc
##
val variable_a = 0;

##
# variable_b doc, variable_c doc
##
val [variable_b, variable_c] = [1,2];
        ",
            vec![
                ("variable_a", 1, ValVariable, Some("variable_a doc")),
                (
                    "variable_b",
                    2,
                    ValVariable,
                    Some("variable_b doc, variable_c doc"),
                ),
                (
                    "variable_c",
                    3,
                    ValVariable,
                    Some("variable_b doc, variable_c doc"),
                ),
            ],
        ),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0;
        let expected_symbols: &Vec<(&str, usize, SymbolKind, Option<&str>)> = &testcase.1;

        let lexer = Lexer::new(input.to_string());
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let collector = match SymbolCollector::collect_from_program(&program) {
            Ok(collector) => collector,
            Err(_) => {
                assert!(false);
                return;
            }
        };

        assert_eq!(collector.table.symbol_map.len(), expected_symbols.len());

        expected_symbols.iter().for_each(|expected| {
            let expected_name = expected.0;
            let expected_symbol_id = SymbolID(expected.1 as u64);
            let expected_symbol_kind = &expected.2;
            let expected_doc = expected.3;

            let symbol = match collector.table.symbol_map.get(&expected_symbol_id) {
                Some(symbol) => symbol,
                None => {
                    assert!(false);
                    return;
                }
            };

            assert_eq!(expected_doc.is_some(), symbol.doc.is_some());

            if let Some(doc) = expected_doc
                && let Some(symbol_doc) = &symbol.doc
            {
                assert_eq!(doc, symbol_doc.raw_content);
            }

            assert_eq!(expected_name, symbol.name);
            assert_eq!(expected_symbol_id, symbol.id);
            assert_eq!(*expected_symbol_kind, symbol.kind);
        });
    });
}

#[test]
pub fn test_enum_statement_symbols() {
    let testcases = [
        (
            "enum State{};",
            vec![("State", 1, SymbolKind::Enum, None, None)],
        ),
        (
            "
##
# enum documentation
##
enum State{};",
            vec![(
                "State",
                1,
                SymbolKind::Enum,
                Some("enum documentation"),
                None,
            )],
        ),
        (
            "enum Action{Read; Write;};",
            vec![
                ("Action", 1, SymbolKind::Enum, None, None),
                ("Read", 2, SymbolKind::EnumVariant, None, Some(1)),
                ("Write", 3, SymbolKind::EnumVariant, None, Some(1)),
            ],
        ),
        (
            "
##
# enum documentation
##
enum Action{Read; Write;};",
            vec![
                (
                    "Action",
                    1,
                    SymbolKind::Enum,
                    Some("enum documentation"),
                    None,
                ),
                ("Read", 2, SymbolKind::EnumVariant, None, Some(1)),
                ("Write", 3, SymbolKind::EnumVariant, None, Some(1)),
            ],
        ),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0;
        let expected_symbols: &Vec<(&str, usize, SymbolKind, Option<&str>, Option<u64>)> =
            &testcase.1;

        let lexer = Lexer::new(input.to_string());
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let collector = match SymbolCollector::collect_from_program(&program) {
            Ok(collector) => collector,
            Err(_) => {
                assert!(false);
                return;
            }
        };

        assert_eq!(collector.table.symbol_map.len(), expected_symbols.len());

        expected_symbols.iter().for_each(|expected| {
            let expected_name = expected.0;
            let expected_symbol_id = SymbolID(expected.1 as u64);
            let expected_symbol_kind = &expected.2;
            let expected_doc = expected.3;
            let expected_owner = expected.4;

            let symbol = match collector.table.symbol_map.get(&expected_symbol_id) {
                Some(symbol) => symbol,
                None => {
                    assert!(false);
                    return;
                }
            };

            assert_eq!(expected_doc.is_some(), symbol.doc.is_some());

            if let Some(doc) = expected_doc
                && let Some(symbol_doc) = &symbol.doc
            {
                assert_eq!(doc, symbol_doc.raw_content);
            }

            assert_eq!(expected_owner.is_some(), symbol.owner.is_some());

            if let Some(owner) = expected_owner
                && let Some(symbol_owner) = &symbol.owner
            {
                assert_eq!(SymbolID(owner), *symbol_owner);
            }

            assert_eq!(expected_name, symbol.name);
            assert_eq!(expected_symbol_id, symbol.id);
            assert_eq!(*expected_symbol_kind, symbol.kind);
        });
    });
}
