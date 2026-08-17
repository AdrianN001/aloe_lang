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
            vec![("variable", 1, LetVariable)],
        ),
        (
            "let [variable_a, variable_b] = [0,1];",
            vec![
                ("variable_a", 1, LetVariable),
                ("variable_b", 2, LetVariable),
            ],
        ),
        (
            "
        let variable_a = 0;
        let [variable_b, variable_c] = [1,2];
        ",
            vec![
                ("variable_a", 1, LetVariable),
                ("variable_b", 2, LetVariable),
                ("variable_c", 3, LetVariable),
            ],
        ),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0;
        let expected_symbols: &Vec<(&str, usize, SymbolKind)> = &testcase.1;

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

            let symbol = match collector.table.symbol_map.get(&expected_symbol_id) {
                Some(symbol) => symbol,
                None => {
                    assert!(false);
                    return;
                }
            };

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
            vec![("variable", 1, ValVariable)],
        ),
        (
            "val [variable_a, variable_b] = [0,1];",
            vec![
                ("variable_a", 1, ValVariable),
                ("variable_b", 2, ValVariable),
            ],
        ),
        (
            "
        val variable_a = 0;
        val [variable_b, variable_c] = [1,2];
        ",
            vec![
                ("variable_a", 1, ValVariable),
                ("variable_b", 2, ValVariable),
                ("variable_c", 3, ValVariable),
            ],
        ),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0;
        let expected_symbols: &Vec<(&str, usize, SymbolKind)> = &testcase.1;

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

            let symbol = match collector.table.symbol_map.get(&expected_symbol_id) {
                Some(symbol) => symbol,
                None => {
                    assert!(false);
                    return;
                }
            };

            assert_eq!(expected_name, symbol.name);
            assert_eq!(expected_symbol_id, symbol.id);
            assert_eq!(*expected_symbol_kind, symbol.kind);
        });
    });
}
