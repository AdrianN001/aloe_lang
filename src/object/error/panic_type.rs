#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PanicType {
    IndexOutOfBound,
    Overflow,
    UnknownIdentifier,
    UnknownMethod,
    UnknownAttribute,

    WrongType,
    WrongIndexType,
    WrongArgumentType,
    WrongArgumentCount,
    Destructuring,

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

    MethodMissingThis,
    ConstructorIsNotAMethod,

    NonAwaitableObjectWasAwaited,
    //memory: MultipleAwaitInOneStatement,
    AwaitedInNonAsyncContext,

    WrongSyntax,

    UTF8Conversion, //TODO: error
}
