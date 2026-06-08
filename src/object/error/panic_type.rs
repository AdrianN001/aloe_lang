#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PanicType {
    IndexOutOfBound,
    KeyNotFound,
    Overflow,
    UnknownIdentifier,
    UnknownMethod,
    UnknownAttribute,
    UnknownEnumValue,

    Assertion,

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

impl PanicType {
    pub fn from_str(string: &str) -> Option<PanicType> {
        match string {
            "IndexOutOfBound" => Some(PanicType::IndexOutOfBound),
            "KeyNotFound" => Some(PanicType::KeyNotFound),
            "Overflow" => Some(PanicType::Overflow),
            "UnknownIdentifier" => Some(PanicType::UnknownIdentifier),
            "UnknownMethod" => Some(PanicType::UnknownMethod),
            "UnknownAttribute" => Some(PanicType::UnknownAttribute),
            "UnknownEnumValue" => Some(PanicType::UnknownEnumValue),
            "Assertion" => Some(PanicType::Assertion),

            "WrongType" => Some(PanicType::WrongType),
            "WrongIndexType" => Some(PanicType::WrongIndexType),
            "WrongArgumentType" => Some(PanicType::WrongArgumentType),
            "WrongArgumentCount" => Some(PanicType::WrongArgumentCount),
            "Destructuring" => Some(PanicType::Destructuring),

            "DivisionByNull" => Some(PanicType::DivisionByNull),

            "IllegalTypeCasting" => Some(PanicType::IllegalTypeCasting),

            "UnexpectedKeyword" => Some(PanicType::UnexpectedKeyword),
            "ImportUnsupported" => Some(PanicType::ImportUnsupported),

            "MissingIdentifier" => Some(PanicType::MissingIdentifier),

            "ReturnFromNonfunctionalContext" => Some(PanicType::ReturnFromNonfunctionalContext),
            "PropagationFromNonfunctionalContext" => {
                Some(PanicType::PropagationFromNonfunctionalContext)
            }

            "VariableIsNotDeclared" => Some(PanicType::VariableIsNotDeclared),
            "UnexpectedRValue" => Some(PanicType::UnexpectedRValue),

            "NonFunctionalObjectCalled" => Some(PanicType::NonfunctionalObjectCalled),
            "ObjectNotIterable" => Some(PanicType::ObjectNotIterable),

            "OperatorIsNotSupported" => Some(PanicType::OperatorIsNotSupported),
            "ObjectNotHashable" => Some(PanicType::ObjectNotHashable),

            "IdentifierNotFoundInModule" => Some(PanicType::IdentifierNotFoundInModule),
            "ModuleCouldNotBeLoaded" => Some(PanicType::ModuleCouldNotBeLoaded),
            "ModuleCouldNotBeExecuted" => Some(PanicType::ModuleCouldNotBeExecuted),

            "IllegalExpression" => Some(PanicType::IllegalExpression),
            "Propagation" => Some(PanicType::Propagation),

            "MethodMissingThis" => Some(PanicType::MethodMissingThis),
            "ConstructorIsNotAMethod" => Some(PanicType::ConstructorIsNotAMethod),

            "NonAwaitableObjectWasAwaited" => Some(PanicType::NonAwaitableObjectWasAwaited),
            "AwaitedInNonAsyncContext" => Some(PanicType::AwaitedInNonAsyncContext),
            "WrongSyntax" => Some(PanicType::WrongSyntax),
            "UTF8Conversion" => Some(PanicType::UTF8Conversion),

            _ => None,
        }
    }
}
