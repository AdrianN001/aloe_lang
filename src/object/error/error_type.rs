use std::fmt::{self};

#[derive(PartialEq, Eq, Clone, Debug)]
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
