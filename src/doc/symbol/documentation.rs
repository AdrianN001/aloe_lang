use std::unimplemented;

use serde::{Deserialize, Serialize};

use crate::{
    ast::{Parser, syntax_error_report::syntax_error::SyntaxError},
    doc::symbol::doc_module::DocModule,
    lexer::Lexer,
    module::Module,
    symbol::collector::symbol_collector::SymbolCollector,
};

#[derive(Serialize, Deserialize)]
pub struct Documentation {
    pub modules: Vec<DocModule>,
}

impl Documentation {
    pub fn from_single_input(input: &str) -> Result<Self, SyntaxError> {
        let lexer = Lexer::new(input.to_string());
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().unwrap();

        let collector = SymbolCollector::collect_from_program(&program)?;

        let doc_module = DocModule::from_symbol_collector("single unit", &collector);

        Ok(Self {
            modules: vec![doc_module],
        })
    }

    pub fn from_single_file(path: &str) -> Result<Self, SyntaxError> {
        let input = Module::read_source_file(path);
        Self::from_single_input(&input)
    }

    pub fn from_project(_path: &str) -> Self {
        unimplemented!()
    }
}
