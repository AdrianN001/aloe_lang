use core::fmt;

use crate::{ast::expression::Expression, token::token_type::TokenType};

#[derive(Debug, Clone)]
pub enum SyntaxError {
    UnexpectedKeyword(TokenType, TokenType), // UnexpectedKeyword(expected, got)
    UnexpectedToken(TokenType, TokenType),   // UnexpectedToken(expected, got)
    UnexpectedTokenWithMultipleChoice(Vec<TokenType>, TokenType),
    UnexpectedExpression(Vec<&'static str>, Expression),

    UnexpectedSemicolon,

    UnexpectedTokenInStruct(Vec<TokenType>, TokenType, String), //UnexpectedTokenInStruct(expected, got, StructName),
    UnexpectedTokenAfterAsync(Vec<TokenType>, TokenType),

    UnexpectedTokenInForLoopHead(Vec<&'static str>, TokenType),

    MethodCallWithoutIdentifier(Expression), // received Expression
    MemberExpressionWithoutAttributeOrMethodCall(Expression), // received Expression,

    IntegerCanNotBeParsed(String),

    TokenCanNotBeParsedCorrectly(TokenType),
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            SyntaxError::UnexpectedKeyword(expected, got) => {
                format!("Expected Keyword: '{}', but got: '{}'", expected, got)
            }
            SyntaxError::UnexpectedToken(expected, got) => {
                format!("Expected '{}', but got: '{}'", expected, got)
            }
            SyntaxError::UnexpectedTokenWithMultipleChoice(expected_list, got) => {
                let expected_str = expected_list
                    .iter()
                    .map(|token| token.to_string())
                    .collect::<Vec<String>>()
                    .join(" or ");
                format!("Expected '{}', but got: '{}'", expected_str, got)
            }
            SyntaxError::UnexpectedExpression(expected_list, got) => {
                let expected_str = expected_list.join(" or ");
                format!(
                    "Expected '{}', but got: '{}'",
                    expected_str,
                    got.to_string()
                )
            }
            SyntaxError::UnexpectedSemicolon => String::from("Unexpeced semicolon (;)"),
            SyntaxError::UnexpectedTokenInStruct(expected_list, got, struct_name) => {
                let expected_str = expected_list
                    .iter()
                    .map(|token| token.to_string())
                    .collect::<Vec<String>>()
                    .join(" or ");

                format!(
                    "Expected '{}' in struct '{}', but got: {}",
                    expected_str, struct_name, got
                )
            }
            SyntaxError::UnexpectedTokenAfterAsync(expected_list, got) => {
                let expected_str = expected_list
                    .iter()
                    .map(|token| token.to_string())
                    .collect::<Vec<String>>()
                    .join(" or ");

                format!(
                    "Expected '{}' after async keyword, but got: {}",
                    expected_str, got
                )
            }
            SyntaxError::UnexpectedTokenInForLoopHead(expected_list, got) => {
                let expected_str = expected_list.join(" or ");

                format!(
                    "Expected '{}' in for loop head, got: '{}'",
                    expected_str, got
                )
            }
            SyntaxError::MethodCallWithoutIdentifier(_) => {
                String::from("Method call without identifier is not allowed.")
            }
            SyntaxError::MemberExpressionWithoutAttributeOrMethodCall(_) => {
                String::from("A method expression must have an attribute or a method call in it")
            }
            SyntaxError::IntegerCanNotBeParsed(received_expression) => {
                format!("{} can not be parsed into an integer.", received_expression)
            }
            SyntaxError::TokenCanNotBeParsedCorrectly(received_token) => {
                format!("'{}' can not be parsed correctly", received_token)
            }
        };
        write!(f, "{}", text)
    }
}
