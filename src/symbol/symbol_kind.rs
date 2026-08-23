use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum SymbolKind {
    LetVariable,
    ValVariable,
    Function,
    AsyncFunction,
    FunctionParameter,
    ForLoopVariable,
    Struct,
    StructAttribute,
    StructMethod,
    StructAsyncMethod,
    Enum,
    EnumVariant,
    Module,
}

impl fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind_str = match self {
            SymbolKind::LetVariable => "let variable",
            SymbolKind::ValVariable => "val variable",
            SymbolKind::Function => "function",
            SymbolKind::AsyncFunction => "async function",
            SymbolKind::FunctionParameter => "function parameter",
            SymbolKind::ForLoopVariable => "for loop variable",
            SymbolKind::Struct => "struct",
            SymbolKind::StructAttribute => "struct attribute",
            SymbolKind::StructMethod => "struct method",
            SymbolKind::StructAsyncMethod => "struct async method",
            SymbolKind::Enum => "enum",
            SymbolKind::EnumVariant => "enum variant",
            SymbolKind::Module => "module",
        };
        write!(f, "{}", kind_str)
    }
}