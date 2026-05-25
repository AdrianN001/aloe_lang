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
