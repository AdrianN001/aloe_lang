use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::{Expression, for_loop::ForLoopExpression, identifier::Identifier},
    object::{
        Object, ObjectRef,
        error::panic_type::PanicType,
        panic_obj::{PanicObj, RuntimeSignal},
        stack_environment::{EnvRef, StackEnvironment},
        state::StateRef,
    },
};

impl ForLoopExpression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        let new_environment = Rc::new(RefCell::new(StackEnvironment::new_enclosed(
            environ.clone(),
            if let Some(variable) = &self.variable
                && let Some(iterator) = &self.iterator
            {
                format!(
                    "for {} <- {} {{...}}",
                    variable.to_string(),
                    iterator.to_string()
                )
            } else {
                "for {...}".to_string()
            },
        )));

        {
            state.borrow_mut().set_current_line(self.token.line_number);
        }

        {
            let mut env_borrow = new_environment.borrow_mut();
            env_borrow.set_loop_context(true);
        }

        if let Some(variable) = &self.variable
            && let Some(iteratable) = &self.iterator
        {
            return match (&**variable, &**iteratable) {
                (Expression::Identifier(identifier), iterable_expression) => self
                    .evaluate_normal_for_loop(
                        new_environment,
                        identifier,
                        iterable_expression,
                        state,
                    ),
                _ => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::MissingIdentifier,
                        "expected identifier for 'for loop', got nothing",
                        state.clone(),
                    )));
                }
            };
        }

        self.evaluate_conditionless_for_loop(new_environment, state)
    }

    fn evaluate_normal_for_loop(
        &self,
        environ: EnvRef,
        variable: &Identifier,
        iterable: &Expression,
        state: StateRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        let provided_object = iterable.evaluate(environ.clone(), state.clone())?;

        let mut iterator = match &*provided_object.borrow() {
            Object::Iterator(iterator) => *iterator.clone(),
            Object::Array(arr) => arr.build_iterator(),
            Object::String(str) => str.build_char_iterator(),
            Object::HashMap(hashmap) => hashmap.build_iterator(),
            _ => {
                return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                    PanicType::ObjectNotIterable,
                    "value provided to for loop is not an iterator",
                    state.clone(),
                )));
            }
        };

        while let Some(current_value) = iterator._next() {
            environ.borrow_mut().set(&variable.value, current_value);

            for statement in &self.block.statements {
                let _ = match statement.evaluate(environ.clone(), state.clone()) {
                    Ok(result) => result,
                    Err(RuntimeSignal::Break(val)) => return Ok(val),
                    Err(RuntimeSignal::Continue) => break,
                    other_err => return other_err,
                };
            }
        }

        Ok(Rc::new(RefCell::new(Object::NULL_OBJECT)))
    }

    fn evaluate_conditionless_for_loop(
        &self,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        loop {
            for statement in &self.block.statements {
                let _ = match statement.evaluate(environ.clone(), state.clone()) {
                    Ok(result) => result,
                    Err(RuntimeSignal::Break(val)) => return Ok(val),
                    Err(RuntimeSignal::Continue) => break,
                    other_err => return other_err,
                };
            }
        }
    }
}
