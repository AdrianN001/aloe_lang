use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{expression::identifier::Identifier, statement::block_statement::BlockStatement},
    object::{
        Object, ObjectRef,
        error::panic_type::PanicType,
        future::{FutureObj, future_kind::FutureKind, future_state::FutureState, task::Task},
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

        let env = self.extend_environment_with_args(name_of_the_function.clone(), arguments);

        let new_future_ref = new_objectref(Object::Future(FutureObj::new(FutureState::Invalid)));

        let new_task_ref = Rc::new(RefCell::new(Task {
            statement_index: 0_usize,
            statements: self.body.statements.clone(),
            name: name_of_the_function,
            kind: None,
            last_object: Some(new_objectref(Object::NULL_OBJECT)),
            environ: env,
            state: state.clone(),
            result_future: Some(new_future_ref.clone()),
            ..Default::default()
        }));

        {
            if let Object::Future(future_obj) = &mut *(new_future_ref.borrow_mut()) {
                future_obj.state = FutureState::Pending(FutureKind::Value(new_task_ref));
            }
        }

        Ok(new_future_ref)
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
