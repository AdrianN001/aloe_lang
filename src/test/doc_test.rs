use std::assert_eq;

use crate::{
    ast::Parser,
    doc::symbol::{
        doc_module::{self, DocModule},
        doc_symbol::DocSymbol,
    },
    lexer::Lexer,
    symbol::{
        collector::symbol_collector::SymbolCollector, symbol::SymbolID, symbol_kind::SymbolKind,
    },
};

struct SimplifiedDocSymbol {
    pub name: String,
    pub id: SymbolID,
    pub kind: SymbolKind,
    pub doc: Option<&'static str>,
    pub children: Vec<SimplifiedDocSymbol>,
}
#[test]
fn test_doc_module_parsing() {
    let testcases = [
        (
            "
struct Person{
    fun sync_method(){}
    async fun async_method(){}
}
",
            vec![SimplifiedDocSymbol {
                name: String::from("Person"),
                id: SymbolID(1),
                kind: SymbolKind::Struct,
                doc: None,
                children: vec![
                    SimplifiedDocSymbol {
                        name: String::from("sync_method"),
                        id: SymbolID(2),
                        kind: SymbolKind::StructMethod,
                        doc: None,
                        children: vec![],
                    },
                    SimplifiedDocSymbol {
                        name: String::from("async_method"),
                        id: SymbolID(3),
                        kind: SymbolKind::StructAsyncMethod,
                        doc: None,
                        children: vec![],
                    },
                ],
            }],
        ),
        (
            "
struct Person{
    fun sync_method(){}
    async fun async_method(){}
    fun sync_method_with_parameter(a, b){}
}
",
            vec![SimplifiedDocSymbol {
                name: String::from("Person"),
                id: SymbolID(1),
                kind: SymbolKind::Struct,

                doc: None,
                children: vec![
                    SimplifiedDocSymbol {
                        name: String::from("sync_method"),
                        id: SymbolID(2),
                        kind: SymbolKind::StructMethod,

                        doc: None,
                        children: vec![],
                    },
                    SimplifiedDocSymbol {
                        name: String::from("async_method"),
                        id: SymbolID(3),
                        kind: SymbolKind::StructAsyncMethod,

                        doc: None,
                        children: vec![],
                    },
                    SimplifiedDocSymbol {
                        name: String::from("sync_method_with_parameter"),
                        id: SymbolID(4),
                        kind: SymbolKind::StructMethod,

                        doc: None,
                        children: vec![
                            SimplifiedDocSymbol {
                                name: String::from("a"),
                                id: SymbolID(5),
                                kind: SymbolKind::FunctionParameter,

                                doc: None,
                                children: vec![],
                            },
                            SimplifiedDocSymbol {
                                name: String::from("b"),
                                id: SymbolID(6),
                                kind: SymbolKind::FunctionParameter,

                                doc: None,
                                children: vec![],
                            },
                        ],
                    },
                ],
            }],
        ),
        (
            "
struct Person{
    fun sync_method(){}
    async fun async_method(){}
    fun sync_method_with_parameter(a, b){let y = 0;}
}

if(true){let x = 0;}
",
            vec![SimplifiedDocSymbol {
                name: String::from("Person"),
                id: SymbolID(1),
                kind: SymbolKind::Struct,

                doc: None,
                children: vec![
                    SimplifiedDocSymbol {
                        name: String::from("sync_method"),
                        id: SymbolID(2),
                        kind: SymbolKind::StructMethod,

                        doc: None,
                        children: vec![],
                    },
                    SimplifiedDocSymbol {
                        name: String::from("async_method"),
                        id: SymbolID(3),
                        kind: SymbolKind::StructAsyncMethod,

                        doc: None,
                        children: vec![],
                    },
                    SimplifiedDocSymbol {
                        name: String::from("sync_method_with_parameter"),
                        id: SymbolID(4),
                        kind: SymbolKind::StructMethod,
                        doc: None,

                        children: vec![
                            SimplifiedDocSymbol {
                                name: String::from("a"),
                                id: SymbolID(5),
                                kind: SymbolKind::FunctionParameter,

                                doc: None,
                                children: vec![],
                            },
                            SimplifiedDocSymbol {
                                name: String::from("b"),
                                id: SymbolID(6),
                                kind: SymbolKind::FunctionParameter,

                                doc: None,
                                children: vec![],
                            },
                        ],
                    },
                ],
            }],
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
    # async_method documentation
    ##
    async fun async_method(){}
    
    ##
    # sync_method_with_parameter documentation
    ##
    fun sync_method_with_parameter(a, b){}
}
",
            vec![SimplifiedDocSymbol {
                name: String::from("Person"),
                id: SymbolID(1),
                kind: SymbolKind::Struct,

                doc: Some("Person documentation"),
                children: vec![
                    SimplifiedDocSymbol {
                        name: String::from("sync_method"),
                        id: SymbolID(2),
                        kind: SymbolKind::StructMethod,

                        doc: Some("sync_method documentation"),
                        children: vec![],
                    },
                    SimplifiedDocSymbol {
                        name: String::from("async_method"),
                        id: SymbolID(3),
                        kind: SymbolKind::StructAsyncMethod,

                        doc: Some("async_method documentation"),
                        children: vec![],
                    },
                    SimplifiedDocSymbol {
                        name: String::from("sync_method_with_parameter"),
                        id: SymbolID(4),
                        kind: SymbolKind::StructMethod,

                        doc: Some("sync_method_with_parameter documentation"),
                        children: vec![
                            SimplifiedDocSymbol {
                                name: String::from("a"),
                                id: SymbolID(5),
                                kind: SymbolKind::FunctionParameter,

                                doc: None,
                                children: vec![],
                            },
                            SimplifiedDocSymbol {
                                name: String::from("b"),
                                id: SymbolID(6),
                                kind: SymbolKind::FunctionParameter,

                                doc: None,
                                children: vec![],
                            },
                        ],
                    },
                ],
            }],
        ),
    ];

    testcases.iter().for_each(|testcase| {
        let input = testcase.0;
        let expected_symbols: &Vec<SimplifiedDocSymbol> = &testcase.1;

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

        let doc_module = DocModule::from_symbol_collector("test_name", &collector);

        assert_eq!(doc_module.root_symbols.len(), expected_symbols.len());

        compare_root_symbol_with_expected(&doc_module.root_symbols, expected_symbols);
    });
}

fn compare_root_symbol_with_expected(
    root_symbols: &Vec<DocSymbol>,
    expected_symbols: &Vec<SimplifiedDocSymbol>,
) {
    root_symbols
        .iter()
        .zip(expected_symbols)
        .for_each(|(doc_module_symbol, expected_symbol)| {
            assert_eq!(doc_module_symbol.name, expected_symbol.name);
            assert_eq!(doc_module_symbol.id, expected_symbol.id);
            assert_eq!(doc_module_symbol.kind, expected_symbol.kind);

            assert_eq!(
                doc_module_symbol.doc.is_none(),
                expected_symbol.doc.is_none()
            );
            if let Some(doc_module_symbol) = &doc_module_symbol.doc
                && let Some(expected_doc) = &expected_symbol.doc
            {
                assert_eq!(doc_module_symbol.raw_content, *expected_doc);
            }

            assert_eq!(
                doc_module_symbol.children.len(),
                expected_symbol.children.len()
            );

            compare_root_symbol_with_expected(
                &doc_module_symbol.children,
                &expected_symbol.children,
            );
        });
}
