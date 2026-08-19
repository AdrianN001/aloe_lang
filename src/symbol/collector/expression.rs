use crate::{
    ast::expression::if_expression::IfExpression,
    symbol::collector::symbol_collector::SymbolCollector,
};

impl SymbolCollector {
    pub fn handle_if_expression(&mut self, _expr: IfExpression) {}
}
