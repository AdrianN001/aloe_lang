#[derive(PartialEq, Eq, Clone)]
pub enum ErrorType {
    IndexOutOfBound,
    IllegalCast,

    RangeInput,

    ObjectIsNotHashable,

    WrongArgumentType,
    WrongArgumentCount,

    FunctionHasMismatchingNumberOfParameters,

    ErrorFromPanic,

    IteratorRanOut,

    CustomError(),
}
