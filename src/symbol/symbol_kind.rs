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
