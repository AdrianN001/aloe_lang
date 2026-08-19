use crate::{
    ast::{
        expression::{
            Expression, for_loop::ForLoopExpression, if_expression::IfExpression,
            while_loop::WhileLoopExpression,
        },
        syntax_error_report::syntax_error::SyntaxError,
    },
    symbol::{
        collector::symbol_collector::SymbolCollector, symbol::Symbol, symbol_kind::SymbolKind,
    },
};

impl SymbolCollector {
    pub fn handle_if_expression(&mut self, expr: &IfExpression) -> Result<(), SyntaxError> {
        // main
        self.new_scope();
        for main_statements in &expr.consequence.statements {
            self.collect_statement(main_statements)?;
        }
        self.previous_scope().unwrap();

        // alternatives
        for (_, alternative_block) in &expr.alternatives {
            self.new_scope();

            for stmt in &alternative_block.statements {
                self.collect_statement(stmt)?;
            }

            self.previous_scope().unwrap();
        }

        // else
        if let Some(else_block) = &expr.else_block {
            self.new_scope();

            for stmt in &else_block.statements {
                self.collect_statement(stmt)?;
            }

            self.previous_scope().unwrap();
        }

        Ok(())
    }

    pub fn handle_while_expression(
        &mut self,
        expr: &WhileLoopExpression,
    ) -> Result<(), SyntaxError> {
        self.new_scope();

        for stmt in &expr.block.statements {
            self.collect_statement(stmt)?;
        }

        self.previous_scope().unwrap();

        Ok(())
    }

    pub fn handle_for_expression(&mut self, expr: &ForLoopExpression) -> Result<(), SyntaxError> {
        self.new_scope();

        if let Some(variable) = &expr.variable {
            let variable_name = match &**variable {
                Expression::Identifier(identifier) => identifier.value.clone(),
                other => {
                    return Err(SyntaxError::UnexpectedExpression(
                        vec!["identifier"],
                        other.clone(),
                        expr.token.line_number,
                    ));
                }
            };

            self.register_for_loop_variable(variable_name);
        }

        for stmt in &expr.block.statements {
            self.collect_statement(stmt)?;
        }

        self.previous_scope().unwrap();

        Ok(())
    }

    fn register_for_loop_variable(&mut self, name: String) {
        let new_symbol_id = self.table.generate_symbol_id();
        let new_symbol = Symbol {
            id: new_symbol_id,
            name: name.to_string(),
            kind: SymbolKind::ForLoopVariable,
            owner: None,
            doc: None,
        };

        self.table.register_to_symbolmap(new_symbol.clone());
        self.table
            .register_to_scope(self.current_scope_id, new_symbol);
    }
}
