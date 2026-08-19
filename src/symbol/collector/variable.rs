use std::unreachable;

use crate::{
    ast::{
        expression::Expression,
        statement::{let_statement::LetStatement, val_statement::ValStatement},
        syntax_error_report::syntax_error::SyntaxError,
    },
    doc::doc_comment::DocComment,
    symbol::{
        collector::symbol_collector::SymbolCollector, symbol::Symbol, symbol_kind::SymbolKind,
    },
};

impl SymbolCollector {
    pub fn handle_let_statement(&mut self, let_stmt: &LetStatement) -> Result<(), SyntaxError> {
        let value_assignment_expr = match &let_stmt.assignment {
            Expression::ValueAssign(value_assign) => value_assign,
            _ => unreachable!(),
        };
        match &*value_assignment_expr.left {
            //multiple
            Expression::Array(variable_list_expr) => {
                for expression_element in &variable_list_expr.elements {
                    match expression_element {
                        Expression::Identifier(variable_name_expression) => {
                            self.register_let_variable(
                                &variable_name_expression.value,
                                let_stmt.doc_comment.clone(),
                            );
                        }
                        other => {
                            return Err(SyntaxError::UnexpectedExpression(
                                vec!["Identifier"],
                                other.clone(),
                                let_stmt.token.line_number,
                            ));
                        }
                    }
                }
                Ok(())
            }
            //single
            Expression::Identifier(variable_name_expression) => {
                self.register_let_variable(
                    &variable_name_expression.value,
                    let_stmt.doc_comment.clone(),
                );

                Ok(())
            }

            _ => unreachable!(),
        }
    }

    pub fn handle_val_statement(&mut self, val_stmt: &ValStatement) -> Result<(), SyntaxError> {
        let value_assignment_expr = match &val_stmt.assignment {
            Expression::ValueAssign(value_assign) => value_assign,
            _ => unreachable!(),
        };
        match &*value_assignment_expr.left {
            //multiple
            Expression::Array(variable_list_expr) => {
                for expression_element in &variable_list_expr.elements {
                    match expression_element {
                        Expression::Identifier(variable_name_expression) => {
                            self.register_val_variable(
                                &variable_name_expression.value,
                                val_stmt.doc_comment.clone(),
                            );
                        }
                        other => {
                            return Err(SyntaxError::UnexpectedExpression(
                                vec!["Identifier"],
                                other.clone(),
                                val_stmt.token.line_number,
                            ));
                        }
                    }
                }
                Ok(())
            }
            //single
            Expression::Identifier(variable_name_expression) => {
                self.register_val_variable(
                    &variable_name_expression.value,
                    val_stmt.doc_comment.clone(),
                );
                Ok(())
            }

            _ => unreachable!(),
        }
    }

    fn register_let_variable(&mut self, name: &str, doc_comment: Option<DocComment>) {
        let new_symbol_id = self.table.generate_id();
        let new_symbol = Symbol {
            id: new_symbol_id,
            name: name.to_string(),
            kind: SymbolKind::LetVariable,
            owner: None,
            doc: doc_comment,
        };

        self.table.register(new_symbol);
    }
    fn register_val_variable(&mut self, name: &str, doc_comment: Option<DocComment>) {
        let new_symbol_id = self.table.generate_id();
        let new_symbol = Symbol {
            id: new_symbol_id,
            name: name.to_string(),
            kind: SymbolKind::ValVariable,
            owner: None,
            doc: doc_comment,
        };

        self.table.register(new_symbol);
    }
}
