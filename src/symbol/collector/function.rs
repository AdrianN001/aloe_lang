use crate::{
    ast::{
        statement::function_statement::FunctionStatement,
        syntax_error_report::syntax_error::SyntaxError,
    },
    doc::doc_comment::DocComment,
    symbol::{
        collector::symbol_collector::SymbolCollector,
        symbol::{Symbol, SymbolID},
        symbol_kind::SymbolKind,
    },
};

impl SymbolCollector {
    pub fn handle_function_statement(
        &mut self,
        statement: &FunctionStatement,
    ) -> Result<(), SyntaxError> {
        let function_name = &statement.name;
        let function_symbol_id =
            self.register_function_name(function_name, statement.doc_comment.clone());

        self.new_scope();

        statement
            .parameters
            .iter()
            .for_each(|parameter_identifier| {
                let parameter_name = &parameter_identifier.value;

                self.register_function_parameter(parameter_name, None, function_symbol_id);
            });

        let block = &statement.block;
        for statement in &block.statements {
            self.collect_statement(statement)?;
        }

        match self.previous_scope() {
            Ok(_) => {}
            Err(msg) => panic!("{}", msg),
        };

        Ok(())
    }

    fn register_function_name(&mut self, name: &str, doc: Option<DocComment>) -> SymbolID {
        let new_symbol_id = self.table.generate_symbol_id();
        let new_symbol = Symbol {
            id: new_symbol_id,
            name: name.to_string(),
            kind: SymbolKind::Function,
            owner: None,
            doc,
        };

        self.table.register_to_symbolmap(new_symbol.clone());
        self.table
            .register_to_scope(self.current_scope_id, new_symbol);

        new_symbol_id
    }

    fn register_function_parameter(
        &mut self,
        name: &str,
        doc: Option<DocComment>,
        owner_id: SymbolID,
    ) {
        let new_symbol_id = self.table.generate_symbol_id();
        let new_symbol = Symbol {
            id: new_symbol_id,
            name: name.to_string(),
            kind: SymbolKind::FunctionParameter,
            owner: Some(owner_id),
            doc,
        };

        self.table.register_to_symbolmap(new_symbol.clone());
        self.table
            .register_to_scope(self.current_scope_id, new_symbol);
    }
}
