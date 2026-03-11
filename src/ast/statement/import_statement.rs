use crate::{ast::expression::Expression, token::Token};

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ImportStatement {
    pub token: Token,
    pub identifiers: Vec<Expression>,
    pub module_name: String,
    pub custom_name: Option<String>,
}

impl ImportStatement {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str("import ");
        buffer.push_str(
            &self
                .identifiers
                .iter()
                .map(|expr| expr.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        );
        buffer.push(' ');
        buffer.push_str(&self.module_name);

        if let Some(custom_name) = &self.custom_name {
            buffer.push_str(" into ");
            buffer.push_str(custom_name);
        }

        buffer
    }
}
