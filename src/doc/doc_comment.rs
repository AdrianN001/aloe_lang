pub mod parsed;

use serde::{Deserialize, Serialize};

use crate::{
    doc::doc_comment::parsed::ParsedDocComment, symbol::symbol_kind::SymbolKind, token::Token,
};

#[derive(Hash, PartialOrd, Ord, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct DocComment {
    pub raw_content: String,
    pub parsed_content: Option<ParsedDocComment>,
}

impl DocComment {
    pub fn new_from_token(token: Token) -> Self {
        DocComment {
            raw_content: token.literal,
            parsed_content: None,
        }
    }

    pub fn try_parse(&mut self, symbol_kind: SymbolKind) -> bool {
        match symbol_kind {
            SymbolKind::Struct | SymbolKind::Enum => {
                let parsed = ParsedDocComment::parse_from_struct(&self.raw_content);
                if let Some(parsed) = parsed {
                    self.parsed_content = Some(parsed);
                    true
                } else {
                    false
                }
            }
            SymbolKind::StructMethod
            | SymbolKind::StructAsyncMethod
            | SymbolKind::Function
            | SymbolKind::AsyncFunction => {
                let parsed = ParsedDocComment::parse_from_function(&self.raw_content);
                if let Some(parsed) = parsed {
                    self.parsed_content = Some(parsed);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
