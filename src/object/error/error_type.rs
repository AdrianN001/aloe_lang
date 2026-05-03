#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ErrorType {
    IndexOutOfBound,
    IllegalCast,

    FileIsClosed,

    FileMode,
    FileRead,
    FileSeek,

    RangeInput,

    ObjectIsNotHashable,

    WrongArgumentType,
    WrongArgumentCount,

    ItemNotFound,

    FunctionHasMismatchingNumberOfParameters,

    ErrorFromPanic,

    IteratorRanOut,

    UTFValueCasting,

    SocketAccept,
    SocketRead,

    CustomError(),
}
