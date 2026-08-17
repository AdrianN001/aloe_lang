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
                            if let Some(doc_comment) = &let_stmt.doc_comment {
                                self.register_let_variable_with_doc(
                                    &variable_name_expression.value,
                                    doc_comment,
                                )
                            } else {
                                self.register_let_variable_without_doc(
                                    &variable_name_expression.value,
                                );
                            }
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
                if let Some(doc_comment) = &let_stmt.doc_comment {
                    self.register_let_variable_with_doc(
                        &variable_name_expression.value,
                        doc_comment,
                    )
                } else {
                    self.register_let_variable_without_doc(&variable_name_expression.value);
                }
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
                            if let Some(doc_comment) = &val_stmt.doc_comment {
                                self.register_val_variable_with_doc(
                                    &variable_name_expression.value,
                                    doc_comment,
                                )
                            } else {
                                self.register_val_variable_without_doc(
                                    &variable_name_expression.value,
                                );
                            }
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
                if let Some(doc_comment) = &val_stmt.doc_comment {
                    self.register_val_variable_with_doc(
                        &variable_name_expression.value,
                        doc_comment,
                    )
                } else {
                    self.register_val_variable_without_doc(&variable_name_expression.value);
                }
                Ok(())
            }

            _ => unreachable!(),
        }
    }

    fn register_let_variable_with_doc(&mut self, name: &str, doc_comment: &DocComment) {
        let new_symbol_id = self.table.generate_id();
        let new_symbol = Symbol {
            id: new_symbol_id,
            name: name.to_string(),
            kind: SymbolKind::LetVariable,
            owner: None,
            doc: Some(doc_comment.clone()),
        };

        self.table.register(new_symbol);
    }
    fn register_let_variable_without_doc(&mut self, name: &str) {
        let new_symbol_id = self.table.generate_id();

        let new_symbol = Symbol {
            id: new_symbol_id,
            name: name.to_string(),
            kind: SymbolKind::LetVariable,
            owner: None,
            doc: None,
        };

        self.table.register(new_symbol);
    }
    fn register_val_variable_with_doc(&mut self, name: &str, doc_comment: &DocComment) {
        let new_symbol_id = self.table.generate_id();
        let new_symbol = Symbol {
            id: new_symbol_id,
            name: name.to_string(),
            kind: SymbolKind::ValVariable,
            owner: None,
            doc: Some(doc_comment.clone()),
        };

        self.table.register(new_symbol);
    }
    fn register_val_variable_without_doc(&mut self, name: &str) {
        let new_symbol_id = self.table.generate_id();

        let new_symbol = Symbol {
            id: new_symbol_id,
            name: name.to_string(),
            kind: SymbolKind::ValVariable,
            owner: None,
            doc: None,
        };

        self.table.register(new_symbol);
    }
}
