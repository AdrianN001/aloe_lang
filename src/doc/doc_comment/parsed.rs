use serde::{Deserialize, Serialize};

use crate::ast::Parser;
use crate::ast::expression::Expression;
use crate::ast::precedence::OperationPrecedence;
use crate::lexer::Lexer;

#[derive(Hash, PartialOrd, Ord, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ParsedDocComment {
    Function {
        declaration: String,
        description: String,
        panics: Vec<String>,
        errors: Vec<String>,
        example: Option<String>,
    },

    Struct {
        description: String,
        example: Option<String>,
    },
}

impl ParsedDocComment {
    /// Parses the raw content of a `DocComment` above a struct into a
    /// description and an optional example. Returns `None` if `raw_content`
    /// has no content at all.
    pub fn parse_from_struct(raw_content: &str) -> Option<Self> {
        let lines: Vec<&str> = raw_content.lines().collect();
        if lines.is_empty() {
            return None;
        }

        let example_idx = lines
            .iter()
            .position(|line| line.trim_start().starts_with("example:"));

        let description = lines[..example_idx.unwrap_or(lines.len())]
            .join("\n")
            .trim()
            .to_string();

        let example = example_idx
            .map(|idx| {
                let after_marker = lines[idx].trim_start().trim_start_matches("example:");
                let mut example_lines = vec![after_marker];
                example_lines.extend_from_slice(&lines[idx + 1..]);
                example_lines.join("\n").trim().to_string()
            })
            .filter(|example| !example.is_empty());

        Some(ParsedDocComment::Struct {
            description,
            example,
        })
    }

    /// Parses the raw content of a `DocComment` above a function/method into
    /// declaration, description, panics, errors and an optional example.
    /// Returns `None` if no "panics:" or "errors:" section can be found.
    pub fn parse_from_function(raw_content: &str) -> Option<Self> {
        let lines: Vec<&str> = raw_content.lines().collect();
        if lines.is_empty() {
            return None;
        }

        let declaration = lines[0].trim().to_string();

        let panics_idx = lines
            .iter()
            .position(|line| line.trim_start().starts_with("panics:"))?;
        let errors_idx = lines[panics_idx + 1..]
            .iter()
            .position(|line| line.trim_start().starts_with("errors:"))
            .map(|offset| panics_idx + 1 + offset)?;

        let description = lines[1..panics_idx].join("\n").trim().to_string();

        let example_idx = lines[errors_idx + 1..]
            .iter()
            .position(|line| line.trim_start().starts_with("example:"))
            .map(|offset| errors_idx + 1 + offset);

        let panics_end = example_idx.unwrap_or(lines.len());
        let panics = Self::parse_string_list(&Self::join_section(
            &lines, panics_idx, "panics:", errors_idx,
        ))?;
        let errors = Self::parse_string_list(&Self::join_section(
            &lines, errors_idx, "errors:", panics_end,
        ))?;

        let example = example_idx
            .map(|idx| {
                let after_marker = lines[idx].trim_start().trim_start_matches("example:");
                let mut example_lines = vec![after_marker];
                example_lines.extend_from_slice(&lines[idx + 1..]);
                example_lines.join("\n").trim().to_string()
            })
            .filter(|example| !example.is_empty());

        Some(ParsedDocComment::Function {
            declaration,
            description,
            panics,
            errors,
            example,
        })
    }

    /// Joins the lines of a "panics:"/"errors:" section (from `start` up to,
    /// but excluding, `end`) into a single string, stripping the leading marker.
    fn join_section(lines: &[&str], start: usize, marker: &str, end: usize) -> String {
        let first_line = lines[start].trim_start().trim_start_matches(marker);
        let mut section_lines = vec![first_line];
        section_lines.extend_from_slice(&lines[start + 1..end]);
        section_lines.join("\n")
    }

    /// Parses a text such as `["foo", "bar"]` into a `Vec<String>`.
    /// Returns `None` if the text is not a list expression made of strings.
    fn parse_string_list(text: &str) -> Option<Vec<String>> {
        let mut parser = Parser::new(Lexer::new(text.to_string()));
        let expression = parser.parse_expression(OperationPrecedence::Lowest).ok()?;

        match expression {
            Expression::Array(array_literal) => array_literal
                .elements
                .into_iter()
                .map(|element| match element {
                    Expression::Identifier(string_expr) => Some(string_expr.value),
                    _ => None,
                })
                .collect(),
            _ => None,
        }
    }
}
