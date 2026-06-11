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

    OS,
    EnvironmentVariableNotFound,

    CustomError(String),
}

impl ErrorType {
    pub fn from_str(err_type: &str) -> Self {
        match err_type {
            "IndexOutOfBound" => Self::IndexOutOfBound,
            "IllegalCast" => Self::IllegalCast,
            "FileOpening" => Self::FileOpening,

            "UnknownPanicType" => Self::UnknownPanicType,

            "WrongRadix" => Self::WrongRadix,
            "FileIsClosed" => Self::FileIsClosed,

            "FileMode" => Self::FileMode,
            "FileRead" => Self::FileRead,
            "FileSeek" => Self::FileSeek,

            "RangeInput" => Self::RangeInput,

            "ObjectIsNotHashable" => Self::ObjectIsNotHashable,

            "ItemNotFound" => Self::ItemNotFound,

            "FunctionHasMismatchingNumberOfParameters" => {
                Self::FunctionHasMismatchingNumberOfParameters
            }

            "ErrorFromPanic" => Self::ErrorFromPanic,

            "IO" => Self::IO,

            "IteratorRanOut" => Self::IteratorRanOut,

            "UTFValueCasting" => Self::UTFValueCasting,

            "IllegalAddress" => Self::IllegalAddress,
            "SocketBind" => Self::SocketBind,
            "SocketConnect" => Self::SocketConnect,
            "SocketAccept" => Self::SocketAccept,
            "SocketRead" => Self::SocketRead,
            "SocketWrite" => Self::SocketWrite,
            "SocketClose" => Self::SocketClose,
            "NonBlockChange" => Self::NonBlockChange,

            "PathResolve" => Self::PathResolve,
            "PathParentResolve" => Self::PathParentResolve,
            "PathChildResolve" => Self::PathChildResolve,

            "Command" => Self::Command,

            "OS" => Self::OS,
            "EnvironmentVariableNotFound" => Self::EnvironmentVariableNotFound,

            other_type => Self::CustomError(other_type.to_string()),
        }
    }
}
