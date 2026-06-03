use core::fmt;

use crate::{ast::expression::Expression, token::token_type::TokenType};

#[derive(Debug, Clone)]
pub enum SyntaxError {
    UnexpectedKeyword(TokenType, TokenType, usize), // UnexpectedKeyword(expected, got)
    UnexpectedToken(TokenType, TokenType, usize),   // UnexpectedToken(expected, got)
    UnexpectedTokenWithMultipleChoice(Vec<TokenType>, TokenType, usize),
    UnexpectedExpression(Vec<&'static str>, Expression, usize),

    UnexpectedSemicolon(usize),

    UnexpectedTokenInStruct(Vec<TokenType>, TokenType, String, usize), //UnexpectedTokenInStruct(expected, got, StructName),
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
                    line_number, expected, got
                )
            }
            SyntaxError::UnexpectedToken(expected, got, line_number) => {
                format!(
                    "line {}, expected '{}', but got: '{}'",
                    line_number, expected, got
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
                    line_number, expected_str, got
                )
            }
            SyntaxError::UnexpectedExpression(expected_list, got, line_number) => {
                let expected_str = expected_list.join(" or ");
                format!(
                    "line {}, expected '{}', but got: '{}'",
                    line_number,
                    expected_str,
                    got.to_string()
                )
            }
            SyntaxError::UnexpectedSemicolon(line_number) => {
                format!("line {}, unexpeced semicolon (;)", line_number)
            }
            SyntaxError::UnexpectedTokenInStruct(expected_list, got, struct_name, line_number) => {
                let expected_str = expected_list
                    .iter()
                    .map(|token| token.to_string())
                    .collect::<Vec<String>>()
                    .join(" or ");

                format!(
                    "line {}, expected '{}' in struct '{}', but got: {}",
                    line_number, expected_str, struct_name, got
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
                    line_number, expected_str, got
                )
            }
            SyntaxError::UnexpectedTokenInForLoopHead(expected_list, got, line_number) => {
                let expected_str = expected_list.join(" or ");

                format!(
                    "line {}, expected '{}' in for loop head, got: '{}'",
                    line_number, expected_str, got
                )
            }
            SyntaxError::MethodCallWithoutIdentifier(_, line_number) => {
                format!(
                    "line {}, method call without identifier is not allowed.",
                    line_number
                )
            }
            SyntaxError::MemberExpressionWithoutAttributeOrMethodCall(_, line_number) => {
                format!(
                    "line {}, method expression must have an attribute or a method call in it",
                    line_number
                )
            }
            SyntaxError::IntegerCanNotBeParsed(received_expression, line_number) => {
                format!(
                    "line {}, {} can not be parsed into an integer.",
                    line_number, received_expression
                )
            }
            SyntaxError::FloatCanNotBeParsed(received_expression, line_number) => {
                format!(
                    "line {}, {} can not be parsed into a float.",
                    line_number, received_expression
                )
            }
            SyntaxError::TokenCanNotBeParsedCorrectly(received_token, line_number) => {
                format!(
                    "line {}, '{}' can not be parsed correctly",
                    line_number, received_token
                )
            }
        };
        write!(f, "{}", text)
    }
}
