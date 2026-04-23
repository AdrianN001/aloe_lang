use crate::{ast::expression::Expression, token::Token};

#[derive(Clone, Hash, PartialOrd, Ord, Default, PartialEq, Eq, Debug)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,

    pub question_mark_set: bool,
    pub bang_set: bool,
}

impl CallExpression {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str(&self.function.to_string());

        buffer.push('(');
        buffer.push_str(
            &self
                .arguments
                .iter()
                .map(|argument| argument.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        );
        buffer.push(')');

        if self.question_mark_set {
            buffer.push('?');
        } else if self.bang_set {
            buffer.push('!');
        }

        buffer
    }
}
