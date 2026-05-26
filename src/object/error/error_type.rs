#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ErrorType {
    IndexOutOfBound,
    IllegalCast,
    FileOpening,

    FileIsClosed,

    FileMode,
    FileRead,
    FileSeek,

    RangeInput,

    ObjectIsNotHashable,

    ItemNotFound,

    FunctionHasMismatchingNumberOfParameters,

    ErrorFromPanic,

    IO,

    IteratorRanOut,

    UTFValueCasting,

    SocketBind,
    SocketConnect,
    SocketAccept,
    SocketRead,
    SocketWrite,
    SocketClose,
    NonBlockChange,

    PathResolve,
    PathParentResolve,
    PathChildResolve,

    CustomError(),
}
