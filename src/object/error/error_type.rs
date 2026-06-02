#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ErrorType {
    IndexOutOfBound,
    IllegalCast,
    FileOpening,

    UnknownPanicType,

    WrongRadix,
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

    IllegalAddress,
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

    Command,

    CustomError(),
}
