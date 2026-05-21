use core::fmt;

use crate::{ast::syntax_error_report::syntax_error::SyntaxError, token::Token};

pub mod syntax_error;

#[derive(Clone, Debug)]
pub struct SyntaxErrorReport {
    pub tokens: Vec<Token>,
    pub error: Option<SyntaxError>,
}

impl SyntaxErrorReport {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            error: None,
        }
    }

    pub fn push_token(&mut self, token: &Token) {
        self.tokens.push(token.clone());
    }

    pub fn clear(&mut self) {
        self.tokens.clear();
    }

    pub fn set_error(&mut self, err: SyntaxError) {
        self.error = Some(err);
    }
}

impl fmt::Display for SyntaxErrorReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buff = String::new();
        let err = self.error.clone().expect("already set");

        buff.push_str("\n\tSyntaxError in the followin statement:\n");
        buff.push('\t');
        buff.push('\t');

        self.tokens.iter().for_each(|token| {
            let token_str = &token.literal;
            buff.push_str(token_str);
            buff.push(' ');
        });

        buff.push('\n');
        buff.push('\n');
        buff.push_str(&format!("\tReason:\n\t\t{}", err));

        write!(f, "{}", buff)
    }
}
