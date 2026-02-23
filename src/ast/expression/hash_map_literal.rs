use std::collections::BTreeMap;

use crate::{ast::expression::Expression, token::Token};

#[derive(Default, Hash, PartialOrd, Ord, Clone, PartialEq, Eq)]
pub struct HashMapLiteral {
    pub token: Token,

    pub pairs: BTreeMap<Expression, Expression>,
}

impl HashMapLiteral {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push('{');

        self.pairs
            .iter()
            .map(|(k, v)| k.to_string() + ":" + &v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        buffer.push('}');
        buffer
    }
}
