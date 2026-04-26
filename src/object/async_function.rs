use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{expression::identifier::Identifier, statement::block_statement::BlockStatement},
    object::{
        Object, ObjectRef,
        error::panic_type::PanicType,
        future::{FutureObj, future_state::FutureState, task::Task},
        new_objectref,
        panic_obj::{PanicObj, RuntimeSignal},
        stack_environment::{EnvRef, StackEnvironment},
        state::StateRef,
    },
};

#[derive(PartialEq, Eq, Clone, Debug)]
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

    // Async Function calling

    pub fn apply(
        &self,
        name_of_the_function: String,
        arguments: &[ObjectRef],
        state: StateRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        if arguments.len() != self.parameters.len() {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "expected {} arguments for function '{}()', got: {}",
                    self.parameters.len(),
                    name_of_the_function,
                    arguments.len()
                ),
                state.clone(),
            )));
        }

        let env = self.extend_environment_with_args(name_of_the_function, arguments);

        Ok(new_objectref(Object::Future(FutureObj {
            state: FutureState::Pending(Task {
                statement_index: 0_usize,
                statements: self.body.statements.clone(),
                environ: env,
                state: state.clone(),
            }),
        })))
    }

    fn extend_environment_with_args(
        &self,
        name_of_the_function: String,
        args: &[ObjectRef],
    ) -> EnvRef {
        let mut new_env =
            StackEnvironment::new_enclosed(self.env.clone(), format!("{}()", name_of_the_function));

        self.parameters
            .iter()
            .enumerate()
            .for_each(|(indx, parameter)| {
                new_env.set_to_lowest_level(&parameter.value, args[indx].clone());
            });

        Rc::new(RefCell::new(new_env))
    }
}
