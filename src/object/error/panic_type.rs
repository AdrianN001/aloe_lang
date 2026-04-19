#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PanicType {
    IndexOutOfBound,
    UnknownIdentifier,
    UnknownMethod,
    UnknownAttribute,

    WrongArgumentType,
    WrongArgumentCount,

    DivisionByNull,

    IllegalTypeCasting,

    UnexpectedKeyword,
    ImportUnsupported,

    MissingIdentifier,

    ReturnFromNonfunctionalContext,
    PropagationFromNonfunctionalContext,

    VariableIsNotDeclared,
    UnexpectedRValue,

    NonfunctionalObjectCalled,
    ObjectNotIterable,

    OperatorIsNotSupported,
    ObjectNotHashable,

    IdentifierNotFoundInModule,
    ModuleCouldNotBeLoaded,
    ModuleCouldNotBeExecuted,

    IllegalExpression,
    Propagation,

    FileOpen,

    PathResolve,
    PathChildResolve,

    MethodMissingThis,
    ConstructorIsNotAMethod,
}
