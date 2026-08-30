use std::assert_eq;

use crate::{
    doc::symbol::{doc_symbol::DocSymbol, documentation::Documentation},
    symbol::{symbol::SymbolID, symbol_kind::SymbolKind},
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
##
# Person documentation
##
struct Person{
    fun sync_method(){}
    async fun async_method(){}
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
##
# Person documentation
##
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

                doc: Some("Person documentation"),
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

                doc: Some("Person documentation"),
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

        let documentation = Documentation::from_single_input(input).unwrap();

        let doc_module = &documentation.modules[0];

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

//TODO: overview #[test]
fn test_documentation_export() {
    let testcases = [
        (
            "
    ##
    # Person documentation
    ##
        struct Person{}
        ",
            "{\"modules\":[{\"name\":\"single unit\",\"root_symbols\":[{\"id\":1,\"scope_id\":1,\"name\":\"Person\",\"kind\":\"Struct\",\"doc\":{\"raw_content\":\"Person documentation\",\"parsed_content\":{\"Struct\":{\"description\":\"Person documentation\",\"example\":null}}},\"children\":[]}]}]}",
        ),
        (
            "
            ##
            # State documentation
            ##
            enum State{}",
            "{\"modules\":[{\"name\":\"single unit\",\"root_symbols\":[{\"id\":1,\"scope_id\":1,\"name\":\"State\",\"kind\":\"Enum\",\"doc\":{\"raw_content\":\"State documentation\",\"parsed_content\":{\"Struct\":{\"description\":\"State documentation\",\"example\":null}}},\"children\":[]}]}]}",
        ),
        (
            "
            ##
            # State documentation
            ##
            enum State{Ready;Stopped;}",
            "{\"modules\":[{\"name\":\"single unit\",\"root_symbols\":[{\"id\":1,\"scope_id\":1,\"name\":\"State\",\"kind\":\"Enum\",\"doc\":{\"raw_content\":\"State documentation\",\"parsed_content\":{\"Struct\":{\"description\":\"State documentation\",\"example\":null}}},\"children\":[{\"id\":2,\"scope_id\":1,\"name\":\"Ready\",\"kind\":\"EnumVariant\",\"doc\":null,\"children\":[]},{\"id\":3,\"scope_id\":1,\"name\":\"Stopped\",\"kind\":\"EnumVariant\",\"doc\":null,\"children\":[]}]}]}]}",
        ),
    ];

    testcases.iter().for_each(|(input, expected_json_output)| {
        let documentation = Documentation::from_single_input(input).unwrap();

        let exported_json = match documentation.export_to_json_str() {
            Ok(json) => json,
            Err(_) => {
                assert!(false);
                return;
            }
        };

        assert_eq!(*expected_json_output, exported_json);
    });
}
