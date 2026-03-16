use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::{Expression, call_expression::CallExpression, member::MemberExpression},
    object::{
        Object, ObjectRef, panic_obj::PanicObj, return_value::ReturnValue, stack_environment::EnvRef, state::{self, StateRef}
    },
};

impl MemberExpression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, PanicObj> {
        let left_obj = self.left.evaluate(environ.clone(), state.clone())?;

        if let Object::ReturnVal(ret_val) = &*left_obj.borrow() {
            return Ok(ret_val.unwrap_to_value().clone());
        }

        match &*self.right {
            Expression::Call(call_expr) => {
                let name_of_method = Self::get_call_expressions_identifier(call_expr, state.clone())?;
                let args = call_expr.evaluate_arguments(environ.clone(), state.clone())?;

                let mut obj = left_obj.borrow_mut();

                let return_value =
                    obj.apply_method(&name_of_method, &args, environ.clone(), state.clone());

                let return_value_cloned = return_value.clone();

                if let Object::Err(err) = &*return_value_cloned.borrow() {
                    if call_expr.question_mark_set && !state.borrow().is_function_context() {
                        return Err(
                            PanicObj::new_simple("tried to use ? on a function, without function-context", state.clone())
                        );
                    }
                    if call_expr.bang_set {
                        return Err(PanicObj::from_error(err, state.clone()));
                    } else if call_expr.question_mark_set {
                        return Ok(Rc::new(RefCell::new(Object::ReturnVal(ReturnValue {
                            value: Box::new(return_value.clone()),
                        }))));
                    }
                }

                Ok(return_value)
            }
            Expression::Identifier(identifier_expr) => {
                let name_of_attribute = &identifier_expr.value;

                let obj = left_obj.borrow();
                Ok(obj.apply_attribute(name_of_attribute, environ, state))
            }
            other_expr_type => Err(PanicObj::new(format!(
                "'{}.{}' is illegal.",
                left_obj.borrow().inspect(),
                other_expr_type.to_string()
            ), state.clone())),
        }
    }

    fn get_call_expressions_identifier(call_expr: &CallExpression, state: StateRef) -> Result<String, PanicObj> {
        match &*call_expr.function {
            Expression::Identifier(identifier) => Ok(identifier.value.clone()),
            _ => Err(PanicObj::new(format!("'{}' is illegal", call_expr.to_string()), state.clone())),
        }
    }
}
