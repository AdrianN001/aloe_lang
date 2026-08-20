use crate::{
    ast::{
        expression::Expression, statement::enum_statement::EnumStatement,
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
    pub fn handle_enum_statement(&mut self, statement: &EnumStatement) -> Result<(), SyntaxError> {
        let name = match &statement.name {
            Expression::Identifier(identifier_expr) => &identifier_expr.value,
            other => {
                return Err(SyntaxError::UnexpectedExpression(
                    vec!["Identifier"],
                    other.clone(),
                    statement.token.line_number,
                ));
            }
        };

        let enum_symbol_id = self.register_enum(name, statement.doc_comment.clone());

        for enum_variant_expr in &statement.values {
            match enum_variant_expr {
                Expression::Identifier(identifier_expr) => {
                    self.register_enum_variant(&identifier_expr.value, enum_symbol_id)
                }
                other => {
                    return Err(SyntaxError::UnexpectedExpression(
                        vec!["Identifier"],
                        other.clone(),
                        statement.token.line_number,
                    ));
                }
            }
        }

        Ok(())
    }

    fn register_enum(&mut self, name: &str, doc_comment: Option<DocComment>) -> SymbolID {
        let new_symbol_id = self.table.generate_symbol_id();
        let new_symbol = Symbol {
            id: new_symbol_id,
            scope_id: self.current_scope_id,
            name: name.to_string(),
            kind: SymbolKind::Enum,
            owner: None,
            doc: doc_comment,
        };

        self.table.register_to_symbolmap(new_symbol.clone());
        self.table
            .register_to_scope(self.current_scope_id, new_symbol);
        new_symbol_id
    }

    fn register_enum_variant(&mut self, name: &str, owner_id: SymbolID) {
        let new_symbol_id = self.table.generate_symbol_id();
        let new_symbol = Symbol {
            id: new_symbol_id,
            scope_id: self.current_scope_id,
            name: name.to_string(),
            kind: SymbolKind::EnumVariant,
            owner: Some(owner_id),
            doc: None,
        };

        self.table.register_to_symbolmap(new_symbol.clone());
        self.table
            .register_to_scope(self.current_scope_id, new_symbol);
    }
}
