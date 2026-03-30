#[derive(PartialEq, Eq, Clone)]
pub enum ErrorType {
    IndexOutOfBound,
    IllegalCast,

    FileMode,
    FileRead,
    FileSeek,

    RangeInput,

    ObjectIsNotHashable,

    WrongArgumentType,
    WrongArgumentCount,

    FunctionHasMismatchingNumberOfParameters,

    ErrorFromPanic,

    IteratorRanOut,

    CustomError(),
}
