use std::unreachable;

use crate::{
    ast::{
        expression::Expression,
        statement::{
            Statement, async_function_statement::AsyncFunctionStatement,
            function_statement::FunctionStatement, struct_statement::StructStatement,
        },
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
    pub fn handle_struct_statement(&mut self, stmt: &StructStatement) -> Result<(), SyntaxError> {
        let name = match &stmt.name {
            Expression::Identifier(identifier) => &identifier.value.clone(),
            other_expr => {
                return Err(SyntaxError::UnexpectedExpression(
                    vec!["identifier"],
                    other_expr.clone(),
                    stmt.token.line_number,
                ));
            }
        };
        let struct_symbol_id = self.register_struct_name(name, stmt.doc_comment.clone());

        //attributes
        for attribute_expr in &stmt.attributes {
            match attribute_expr {
                Expression::Identifier(identifier) => {
                    self.register_struct_attribute(&identifier.value, struct_symbol_id);
                }
                _ => unreachable!(),
            }
        }

        //methods
        for method_stmt in &stmt.methods {
            match method_stmt {
                Statement::Function(function_stmt) => {
                    self.handle_function_statement_in_struct(function_stmt, struct_symbol_id)?
                }
                Statement::AsyncFunction(async_function_stmt) => {
                    self.handle_async_function_in_struct(async_function_stmt, struct_symbol_id)?
                }
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    fn register_struct_name(&mut self, name: &str, doc: Option<DocComment>) -> SymbolID {
        let new_symbol_id = self.table.generate_symbol_id();
        let new_symbol = Symbol {
            id: new_symbol_id,
            name: name.to_string(),
            kind: SymbolKind::Struct,
            owner: None,
            doc,
        };

        self.table.register_to_symbolmap(new_symbol.clone());
        self.table
            .register_to_scope(self.current_scope_id, new_symbol);
        new_symbol_id
    }

    fn register_struct_attribute(&mut self, name: &str, struct_id: SymbolID) {
        let new_symbol_id = self.table.generate_symbol_id();
        let new_symbol = Symbol {
            id: new_symbol_id,
            name: name.to_string(),
            kind: SymbolKind::StructAttribute,
            owner: Some(struct_id),
            doc: None,
        };

        self.table.register_to_symbolmap(new_symbol.clone());
        self.table
            .register_to_scope(self.current_scope_id, new_symbol);
    }

    fn handle_function_statement_in_struct(
        &mut self,
        statement: &FunctionStatement,
        struct_id: SymbolID,
    ) -> Result<(), SyntaxError> {
        let method_name = &statement.name;
        let method_symbol_id =
            self.register_method_name(method_name, statement.doc_comment.clone(), struct_id);

        self.new_scope();

        statement
            .parameters
            .iter()
            .for_each(|parameter_identifier| {
                let parameter_name = &parameter_identifier.value;

                self.register_function_parameter(parameter_name, None, method_symbol_id);
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

    fn register_method_name(
        &mut self,
        name: &str,
        doc: Option<DocComment>,
        owner_id: SymbolID,
    ) -> SymbolID {
        let new_symbol_id = self.table.generate_symbol_id();
        let new_symbol = Symbol {
            id: new_symbol_id,
            name: name.to_string(),
            kind: SymbolKind::StructMethod,
            owner: Some(owner_id),
            doc,
        };

        self.table.register_to_symbolmap(new_symbol.clone());
        self.table
            .register_to_scope(self.current_scope_id, new_symbol);

        new_symbol_id
    }

    fn handle_async_function_in_struct(
        &mut self,
        async_function_stmt: &AsyncFunctionStatement,
        struct_id: SymbolID,
    ) -> Result<(), SyntaxError> {
        let function_stmt = match &*async_function_stmt.function {
            Statement::Function(function_stmt) => function_stmt,
            _ => unreachable!(),
        };

        let function_name = &function_stmt.name;
        let function_symbol_id = self.register_async_method_name(
            function_name,
            function_stmt.doc_comment.clone(),
            struct_id,
        );

        self.new_scope();

        function_stmt
            .parameters
            .iter()
            .for_each(|parameter_identifier| {
                let parameter_name = &parameter_identifier.value;

                self.register_function_parameter(parameter_name, None, function_symbol_id);
            });

        let block = &function_stmt.block;
        for statement in &block.statements {
            self.collect_statement(statement)?;
        }

        match self.previous_scope() {
            Ok(_) => {}
            Err(msg) => panic!("{}", msg),
        };

        Ok(())
    }

    fn register_async_method_name(
        &mut self,
        name: &str,
        doc: Option<DocComment>,
        owner_id: SymbolID,
    ) -> SymbolID {
        let new_symbol_id = self.table.generate_symbol_id();
        let new_symbol = Symbol {
            id: new_symbol_id,
            name: name.to_string(),
            kind: SymbolKind::StructAsyncMethod,
            owner: Some(owner_id),
            doc,
        };

        self.table.register_to_symbolmap(new_symbol.clone());
        self.table
            .register_to_scope(self.current_scope_id, new_symbol);

        new_symbol_id
    }
}
