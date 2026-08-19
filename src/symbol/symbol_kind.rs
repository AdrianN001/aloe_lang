#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
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
