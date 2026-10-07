use core::fmt;

use crate::terminal::formatter::TerminalTextFormatter;
use crate::{ast::expression::Expression, token::token_type::TokenType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxError {
    UnexpectedKeyword(TokenType, TokenType, usize), // UnexpectedKeyword(expected, got)
    UnexpectedToken(TokenType, TokenType, usize),   // UnexpectedToken(expected, got)
    UnexpectedTokenWithMultipleChoice(Vec<TokenType>, TokenType, usize),
    UnexpectedExpression(Vec<&'static str>, Expression, usize),

    UnexpectedSemicolon(usize),

    UnexpectedTokenInStruct(Vec<TokenType>, TokenType, String, usize), //UnexpectedTokenInStruct(expected, got, StructName),
    UnexpectedTokenInEnum(Vec<TokenType>, TokenType, String, usize),
    UnexpectedTokenAfterAsync(Vec<TokenType>, TokenType, usize),

    UnexpectedTokenInForLoopHead(Vec<&'static str>, TokenType, usize),

    MethodCallWithoutIdentifier(Expression, usize), // received Expression
    MemberExpressionWithoutAttributeOrMethodCall(Expression, usize), // received Expression,

    IntegerCanNotBeParsed(String, usize),
    FloatCanNotBeParsed(String, usize),

    TokenCanNotBeParsedCorrectly(TokenType, usize),
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            SyntaxError::UnexpectedKeyword(expected, got, line_number) => {
                format!(
                    "line {}, expected Keyword: '{}', but got: '{}'",
                    TerminalTextFormatter::to_bold(&line_number.to_string()),
                    expected,
                    got
                )
            }
            SyntaxError::UnexpectedToken(expected, got, line_number) => {
                format!(
                    "line {}, expected '{}', but got: '{}'",
                    TerminalTextFormatter::to_bold(&line_number.to_string()),
                    expected,
                    got
                )
            }
            SyntaxError::UnexpectedTokenWithMultipleChoice(expected_list, got, line_number) => {
                let expected_str = expected_list
                    .iter()
                    .map(|token| token.to_string())
                    .collect::<Vec<String>>()
                    .join(" or ");
                format!(
                    "line {}, expected '{}', but got: '{}'",
                    TerminalTextFormatter::to_bold(&line_number.to_string()),
                    expected_str,
                    got
                )
            }
            SyntaxError::UnexpectedExpression(expected_list, got, line_number) => {
                let expected_str = expected_list.join(" or ");
                format!(
                    "line {}, expected '{}', but got: '{}'",
                    TerminalTextFormatter::to_bold(&line_number.to_string()),
                    expected_str,
                    got.to_string()
                )
            }
            SyntaxError::UnexpectedSemicolon(line_number) => {
                format!(
                    "line {}, unexpeced semicolon (;)",
                    TerminalTextFormatter::to_bold(&line_number.to_string())
                )
            }
            SyntaxError::UnexpectedTokenInStruct(expected_list, got, struct_name, line_number) => {
                let expected_str = expected_list
                    .iter()
                    .map(|token| token.to_string())
                    .collect::<Vec<String>>()
                    .join(" or ");

                format!(
                    "line {}, expected '{}' in struct '{}', but got: {}",
                    TerminalTextFormatter::to_bold(&line_number.to_string()),
                    expected_str,
                    struct_name,
                    got
                )
            }

            SyntaxError::UnexpectedTokenInEnum(expected_list, got, struct_name, line_number) => {
                let expected_str = expected_list
                    .iter()
                    .map(|token| token.to_string())
                    .collect::<Vec<String>>()
                    .join(" or ");

                format!(
                    "line {}, expected '{}' in enum '{}', but got: {}",
                    TerminalTextFormatter::to_bold(&line_number.to_string()),
                    expected_str,
                    struct_name,
                    got
                )
            }
            SyntaxError::UnexpectedTokenAfterAsync(expected_list, got, line_number) => {
                let expected_str = expected_list
                    .iter()
                    .map(|token| token.to_string())
                    .collect::<Vec<String>>()
                    .join(" or ");

                format!(
                    "line {}, expected '{}' after async keyword, but got: {}",
                    TerminalTextFormatter::to_bold(&line_number.to_string()),
                    expected_str,
                    got
                )
            }
            SyntaxError::UnexpectedTokenInForLoopHead(expected_list, got, line_number) => {
                let expected_str = expected_list.join(" or ");

                format!(
                    "line {}, expected '{}' in for loop head, got: '{}'",
                    TerminalTextFormatter::to_bold(&line_number.to_string()),
                    expected_str,
                    got
                )
            }
            SyntaxError::MethodCallWithoutIdentifier(_, line_number) => {
                format!(
                    "line {}, method call without identifier is not allowed.",
                    TerminalTextFormatter::to_bold(&line_number.to_string())
                )
            }
            SyntaxError::MemberExpressionWithoutAttributeOrMethodCall(_, line_number) => {
                format!(
                    "line {}, method expression must have an attribute or a method call in it",
                    TerminalTextFormatter::to_bold(&line_number.to_string())
                )
            }
            SyntaxError::IntegerCanNotBeParsed(received_expression, line_number) => {
                format!(
                    "line {}, {} can not be parsed into an integer.",
                    TerminalTextFormatter::to_bold(&line_number.to_string()),
                    received_expression
                )
            }
            SyntaxError::FloatCanNotBeParsed(received_expression, line_number) => {
                format!(
                    "line {}, {} can not be parsed into a float.",
                    TerminalTextFormatter::to_bold(&line_number.to_string()),
                    received_expression
                )
            }
            SyntaxError::TokenCanNotBeParsedCorrectly(received_token, line_number) => {
                format!(
                    "line {}, '{}' can not be parsed correctly",
                    TerminalTextFormatter::to_bold(&line_number.to_string()),
                    received_token
                )
            }
        };
        write!(f, "{}", text)
    }
}
