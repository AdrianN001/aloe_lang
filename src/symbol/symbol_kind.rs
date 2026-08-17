#[derive(PartialEq, Eq, Debug)]
pub enum SymbolKind {
    LetVariable,
    ValVariable,
    Function,
    Struct,
    Enum,
    Module,
}
