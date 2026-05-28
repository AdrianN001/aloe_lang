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
    IO,

    SocketBind,
    SocketAccept,
    SocketRead,
    SocketWrite,
    Listener,

    MethodMissingThis,
    ConstructorIsNotAMethod,

    NonAwaitableObjectWasAwaited,
    MultipleAwaitInOneStatement,

    AwaitedInNonAsyncContext,

    WrongSyntax,

    UTF8Conversion,
}
