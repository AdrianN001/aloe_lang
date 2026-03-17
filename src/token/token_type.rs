#[derive(Debug, Hash, PartialOrd, Ord, Clone, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Identifier,
    Integer,
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

    Eq,
    NotEq,
    LT,         // <
    LE,         // <=
    GT,         // >
    GE,         // >=
    Coalescing, // ??

    IteratorAssign, // <-
    Dot,
    Comment, // #

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
    KwTrue,
    KwFalse,
    KwIf,
    KwElif,
    KwElse,
    KwReturn,

    KwImport,
    KwFrom,
    KwInto,

    KwStruct,

    KwDefer,

    KwFor,
    KwBreak,
    KwContinue,
}

pub fn lookup_identifiers(identifier: &str) -> TokenType {
    match identifier {
        "fn" => TokenType::KwFunction,
        "fun" => TokenType::KwFunctionStatement,
        "let" => TokenType::KwLet,
        "true" => TokenType::KwTrue,
        "false" => TokenType::KwFalse,
        "if" => TokenType::KwIf,
        "elif" => TokenType::KwElif,
        "else" => TokenType::KwElse,
        "return" => TokenType::KwReturn,

        "import" => TokenType::KwImport,
        "from" => TokenType::KwFrom,
        "into" => TokenType::KwInto,

        "struct" => TokenType::KwStruct,

        "for" => TokenType::KwFor,
        "break" => TokenType::KwBreak,
        // "defer" => TokenType::KwDefer,
        "continue" => TokenType::KwContinue,

        _ => TokenType::Identifier,
    }
}
