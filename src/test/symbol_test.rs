use std::assert_eq;

use crate::{
    ast::Parser,
    lexer::Lexer,
    symbol::{
        collector::symbol_collector::SymbolCollector,
        scope::ScopeID,
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
        (
            "let x = if (true){ let y = 3; y};",
            vec![
                ("y", 1, SymbolKind::LetVariable, None),
                ("x", 2, SymbolKind::LetVariable, None),
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
        (
            "val x = if (true){ let y = 3; y};",
            vec![
                ("y", 1, SymbolKind::LetVariable, None),
                ("x", 2, SymbolKind::ValVariable, None),
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

#[test]
pub fn test_function_statement_symbols() {
    let testcases = [
        (
            "fun print(){}",
            vec![("print", 1, SymbolKind::Function, None, None)],
        ),
        (
            "
##
# function documentation
##
fun print(){}",
            vec![(
                "print",
                1,
                SymbolKind::Function,
                Some("function documentation"),
                None,
            )],
        ),
        (
            "fun print(parameter_a, parameter_b){}",
            vec![
                ("print", 1, SymbolKind::Function, None, None),
                (
                    "parameter_a",
                    2,
                    SymbolKind::FunctionParameter,
                    None,
                    Some(1),
                ),
                (
                    "parameter_b",
                    3,
                    SymbolKind::FunctionParameter,
                    None,
                    Some(1),
                ),
            ],
        ),
        (
            "
##
# function documentation
##
fun print(parameter_a, parameter_b){}",
            vec![
                (
                    "print",
                    1,
                    SymbolKind::Function,
                    Some("function documentation"),
                    None,
                ),
                (
                    "parameter_a",
                    2,
                    SymbolKind::FunctionParameter,
                    None,
                    Some(1),
                ),
                (
                    "parameter_b",
                    3,
                    SymbolKind::FunctionParameter,
                    None,
                    Some(1),
                ),
            ],
        ),
        (
            "
##
# async function documentation
##
async fun print(parameter_a, parameter_b){}",
            vec![
                (
                    "print",
                    1,
                    SymbolKind::AsyncFunction,
                    Some("async function documentation"),
                    None,
                ),
                (
                    "parameter_a",
                    2,
                    SymbolKind::FunctionParameter,
                    None,
                    Some(1),
                ),
                (
                    "parameter_b",
                    3,
                    SymbolKind::FunctionParameter,
                    None,
                    Some(1),
                ),
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

#[test]
pub fn test_scopes() {
    let testcases = [
        (
            "
fun print(a, b){
    let c = 30;
}
",
            vec![(1, vec!["print"], None), (2, vec!["a", "b", "c"], Some(1))],
        ),
        (
            "
fun print(a, b){
    let c = 30;

    fun nested_print(d){
       let e = 30; 
    }
}
",
            vec![
                (1, vec!["print"], None),
                (2, vec!["a", "b", "c", "nested_print"], Some(1)),
                (3, vec!["d", "e"], Some(2)),
            ],
        ),
        (
            "
fun print(a, b){
    let c = 30;

    fun nested_print(d){
       let e = 30; 
    }

    fun second_nested_print(f,g){
        let [h, i] = [2,3];

        async fun third_nested_print(j,k){
            let l = 0;
        }
    }
}
",
            vec![
                (1, vec!["print"], None),
                (
                    2,
                    vec!["a", "b", "c", "nested_print", "second_nested_print"],
                    Some(1),
                ),
                (3, vec!["d", "e"], Some(2)),
                (4, vec!["f", "g", "h", "i", "third_nested_print"], Some(2)),
                (5, vec!["j", "k", "l"], Some(4)),
            ],
        ),
        (
            "
if (2>3){
    let [a,b,c] = [0,1,2];
}elif(3>2){
    let [d,e,f] = [0,1,2];
}elif(3>2){
    let [g,h,i] = [0,1,2];
}else{
    let [j,k,l] = [0,1,2];

    if (true){
        let [x,y] = [0,1];
    }
}
let z = 0;
",
            vec![
                (1, vec!["z"], None),
                (2, vec!["a", "b", "c"], Some(1)),
                (3, vec!["d", "e", "f"], Some(1)),
                (4, vec!["g", "h", "i"], Some(1)),
                (5, vec!["j", "k", "l"], Some(1)),
                (6, vec!["x", "y"], Some(5)),
            ],
        ),
        (
            "
if (true){
    let [a,b] = [0,1];
    for c <- range(100){
        
    }
}else{
    let d = 0;
}
",
            vec![
                (1, vec![], None),
                (2, vec!["a", "b"], Some(1)),
                (3, vec!["c"], Some(2)),
                (4, vec!["d"], Some(1)),
            ],
        ),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0;
        let expected_scopes: &Vec<(u32, Vec<&str>, Option<u32>)> = &testcase.1;

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

        assert_eq!(collector.table.scopes.len(), expected_scopes.len());

        expected_scopes.iter().for_each(|expected| {
            let expected_scope_id = expected.0;
            let expected_scope_identifiers = &expected.1;
            let expected_scope_owner_id = expected.2;

            let scope = match collector.table.scopes.get(&ScopeID(expected_scope_id)) {
                Some(scope) => scope,
                None => {
                    assert!(false);
                    return;
                }
            };

            assert_eq!(expected_scope_owner_id.is_some(), scope.parent.is_some());

            if let Some(scope_owner_id_test) = &expected_scope_owner_id
                && let Some(scope_owner_id) = &scope.parent
            {
                assert_eq!(ScopeID(*scope_owner_id_test), *scope_owner_id);
            }

            assert_eq!(expected_scope_identifiers.len(), scope.symbols.len());
            for expected_identifier in expected_scope_identifiers {
                assert_eq!(
                    scope.symbols.contains_key(&expected_identifier.to_string()),
                    true
                )
            }
        });
    });
}

#[test]
pub fn test_struct_statement_symbols() {
    let testcases = [
        (
            "struct Person{}",
            vec![("Person", 1, SymbolKind::Struct, None, None)],
        ),
        (
            "
##
# struct documentation
##
struct Person{}",
            vec![(
                "Person",
                1,
                SymbolKind::Struct,
                Some("struct documentation"),
                None,
            )],
        ),
        (
            "struct Person{
                name; age;
            }",
            vec![
                ("Person", 1, SymbolKind::Struct, None, None),
                ("name", 2, SymbolKind::StructAttribute, None, Some(1)),
                ("age", 3, SymbolKind::StructAttribute, None, Some(1)),
            ],
        ),
        (
            "
struct Person{
    fun sync_method(){}
    fun sync_method2(){}
    async fun async_method(){}
}",
            vec![
                ("Person", 1, SymbolKind::Struct, None, None),
                ("sync_method", 2, SymbolKind::StructMethod, None, Some(1)),
                ("sync_method2", 3, SymbolKind::StructMethod, None, Some(1)),
                (
                    "async_method",
                    4,
                    SymbolKind::StructAsyncMethod,
                    None,
                    Some(1),
                ),
            ],
        ),
        (
            "
##
# Person documentation
##
struct Person{
##
# sync_method documentation
##
fun sync_method(){}
##
# sync_method2 documentation
##
fun sync_method2(){}
##
# async_method documentation
##
async fun async_method(){}
}",
            vec![
                (
                    "Person",
                    1,
                    SymbolKind::Struct,
                    Some("Person documentation"),
                    None,
                ),
                (
                    "sync_method",
                    2,
                    SymbolKind::StructMethod,
                    Some("sync_method documentation"),
                    Some(1),
                ),
                (
                    "sync_method2",
                    3,
                    SymbolKind::StructMethod,
                    Some("sync_method2 documentation"),
                    Some(1),
                ),
                (
                    "async_method",
                    4,
                    SymbolKind::StructAsyncMethod,
                    Some("async_method documentation"),
                    Some(1),
                ),
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
