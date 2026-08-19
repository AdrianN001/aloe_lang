#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum SymbolKind {
    LetVariable,
    ValVariable,
    Function,
    FunctionParameter,
    Struct,
    Enum,
    EnumVariant,
    Module,
}
