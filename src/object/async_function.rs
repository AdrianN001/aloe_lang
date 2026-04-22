use crate::{
    ast::{expression::identifier::Identifier, statement::block_statement::BlockStatement},
    object::stack_environment::EnvRef,
};

#[derive(PartialEq, Eq, Clone)]
pub struct AsyncFunction {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: EnvRef,
}

impl AsyncFunction {
    pub fn get_type(&self) -> String {
        "async function".into()
    }

    pub fn inspect(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str("async fn");
        buffer.push('(');

        buffer.push_str(
            &self
                .parameters
                .iter()
                .map(|parameter| parameter.value.clone())
                .collect::<Vec<_>>()
                .join(", "),
        );

        buffer.push_str(") {\n");
        buffer.push_str(&self.body.to_string_for_function());
        buffer.push_str("\n}");

        buffer
    }
}
