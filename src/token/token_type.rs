#[derive(Debug, Hash, PartialOrd, Ord, Clone, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Identifier,
    Integer,
    Float,
    String,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    QuestionMark,
    Exponent, // **
    Modulo,   // %

    LogicalAnd, // &&
    LogicalOr,  // ||
    LogicalXor, // '^'

    BinaryAnd,        // &
    BinaryOr,         // |
    BinaryLeftShift,  // <<
    BinaryRightShift, // >>

    PlusEq,
    MinusEq,
    MulEq,
    DivEq,
    ExpoEq,
    ModEq,
    BinaryAndEq,
    BinaryOrEq,
    BinaryXorEq,
    BinaryLeftShiftEq,
    BinaryRightShiftEq,

    Eq,
    NotEq,
    LT,         // <
    LE,         // <=
    GT,         // >
    GE,         // >=
    Coalescing, // ??

    IteratorAssign, // <-
    Dot,
    Comment,         // #
    ScopeResolution, // ::

    // Delimiters
    Colon,
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // Keywords
    KwFunction,
    KwFunctionStatement,
    KwLet,
    KwVal,
    KwTrue,
    KwFalse,
    KwNull,
    KwIf,
    KwElif,
    KwElse,
    KwReturn,

    KwImport,
    KwFrom,
    KwInto,

    KwStruct,
    KwEnum,

    KwWhile,
    KwFor,
    KwBreak,
    KwContinue,

    KwAsync,
    KwAwait,
    KwLaunch,
}

pub fn lookup_identifiers(identifier: &str) -> TokenType {
    match identifier {
        "fn" => TokenType::KwFunction,
        "fun" => TokenType::KwFunctionStatement,
        "let" => TokenType::KwLet,
        "true" => TokenType::KwTrue,
        "val" => TokenType::KwVal,
        "false" => TokenType::KwFalse,
        "null" => TokenType::KwNull,
        "if" => TokenType::KwIf,
        "elif" => TokenType::KwElif,
        "else" => TokenType::KwElse,
        "return" => TokenType::KwReturn,

        "import" => TokenType::KwImport,
        "from" => TokenType::KwFrom,
        "into" => TokenType::KwInto,

        "struct" => TokenType::KwStruct,
        "enum" => TokenType::KwEnum,

        "while" => TokenType::KwWhile,
        "for" => TokenType::KwFor,
        "break" => TokenType::KwBreak,
        // "defer" => TokenType::KwDefer,
        "continue" => TokenType::KwContinue,

        "async" => TokenType::KwAsync,
        "await" => TokenType::KwAwait,

        "launch" => TokenType::KwLaunch,

        _ => TokenType::Identifier,
    }
}
