use crate::{ast::Parser, token::token_type::TokenType};

impl Parser {
    pub fn create_unexpected_error_feedback(
        &self,
        expected_token_type: &TokenType,
        read_token_type: &TokenType,
    ) -> String {
        format!(
            "expected to be the next token a \"{expected_token_type}\", received: \"{read_token_type}\""
        )
    }
}
