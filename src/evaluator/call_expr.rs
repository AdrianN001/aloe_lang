use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::expression::Expression;
use crate::ast::expression::call_expression::CallExpression;
use crate::object::error::panic_type::PanicType;
use crate::object::panic_obj::{PanicObj, RuntimeSignal};
use crate::object::return_value::ReturnValue;
use crate::object::stack_environment::EnvRef;
use crate::object::state::StateRef;
use crate::object::struct_object::StructObject;
use crate::object::{Object, ObjectRef};

impl CallExpression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        let obj_to_call = self
            .function
            .evaluate(environ.clone(), state.clone())?
            .clone();

        let function_name = match &*self.function {
            Expression::Identifier(identifier) => identifier.value.clone(),
            _ => "(anonymm function)".to_string(),
        };

        let args = self.evaluate_arguments(environ.clone(), state.clone())?;

        // only propagation '?' can it cause. (hopefully)
        for argument in &args {
            if let Object::ReturnVal(_) = &*argument.borrow() {
                return Ok(argument.clone());
            }
        }

        let return_value = match &*obj_to_call.borrow() {
            Object::Func(function) => function.apply(function_name, &args, state.clone()),
            Object::AsyncFunc(async_function) => {
                async_function.apply(function_name, &args, state.clone())
            }
            Object::BuiltIn(built_in_function) => {
                built_in_function.call(&args, environ.clone(), state.clone())
            }
            Object::StructModel(_) => {
                StructObject::create_new_object(obj_to_call.clone(), &args, state.clone())
            }
            other_type => Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::NonfunctionalObjectCalled,
                format!(
                    "'{}' is not callable. It cannot be called.",
                    other_type.inspect()
                ),
                state.clone(),
            ))),
        };

        let ok_return_value = return_value?;

        if let Object::Err(error) = &*ok_return_value.borrow() {
            if self.question_mark_set && !state.borrow().is_function_context() {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::PropagationFromNonfunctionalContext,
                    "tried to use ? on a function, without function-context".to_string(),
                    state.clone(),
                )));
            }

            if self.bang_set {
                return Err(RuntimeSignal::Panic(PanicObj::from_error(error, state)));
            } else if self.question_mark_set {
                return Ok(Rc::new(RefCell::new(Object::ReturnVal(ReturnValue {
                    value: Box::new(ok_return_value.clone()),
                }))));
            }
        }

        Ok(ok_return_value)
    }

    pub fn evaluate_arguments(
        &self,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<Vec<ObjectRef>, RuntimeSignal> {
        self.arguments
            .iter()
            .map(|argument| argument.evaluate(environ.clone(), state.clone()))
            .collect()
    }
}
