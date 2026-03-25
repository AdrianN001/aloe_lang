use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::{Expression, call_expression::CallExpression, member::MemberExpression},
    object::{
        Object, ObjectRef, error::panic_type::PanicType, panic_obj::PanicObj, return_value::ReturnValue, stack_environment::EnvRef, state::StateRef, struct_object::StructObject
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
                let name_of_method =
                    Self::get_call_expressions_identifier(call_expr, state.clone())?;
                let args = call_expr.evaluate_arguments(environ.clone(), state.clone())?;

                if MemberExpression::check_early_for_struct_method_call(left_obj.clone()) {
                    return StructObject::apply_method(
                        &name_of_method,
                        left_obj,
                        &args,
                        environ,
                        state,
                    );
                }

                let mut obj = left_obj.borrow_mut();

                let return_value =
                    obj.apply_method(&name_of_method, &args, environ.clone(), state.clone())?;

                let return_value_cloned = return_value.clone();

                if let Object::Err(err) = &*return_value_cloned.borrow() {
                    if call_expr.question_mark_set && !state.borrow().is_function_context() {
                        return Err(PanicObj::new_simple(
                            PanicType::PropagationFromNonfunctionalContext,
                            "tried to use ? on a function, without function-context",
                            state.clone(),
                        ));
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
                obj.apply_attribute(name_of_attribute, environ, state)
            }
            other_expr_type => Err(PanicObj::new(
                PanicType::IllegalExpression,
                format!(
                    "'{}.{}' is illegal.",
                    left_obj.borrow().inspect(),
                    other_expr_type.to_string()
                ),
                state.clone(),
            )),
        }
    }

    fn check_early_for_struct_method_call(left_obj: ObjectRef) -> bool {
        let left_obj_borrow = left_obj.borrow();

        match &*left_obj_borrow {
            Object::StructObject(_) => return true,
            _ => return false,
        }
    }

    fn get_call_expressions_identifier(
        call_expr: &CallExpression,
        state: StateRef,
    ) -> Result<String, PanicObj> {
        match &*call_expr.function {
            Expression::Identifier(identifier) => Ok(identifier.value.clone()),
            _ => Err(PanicObj::new(
                PanicType::IllegalExpression,
                format!("'{}' is illegal", call_expr.to_string()),
                state.clone(),
            )),
        }
    }

    pub fn evaluate_value_assign(
        &self,
        environ: EnvRef,
        state: StateRef,
        r_value: ObjectRef,
    ) -> Result<ObjectRef, PanicObj> {
        let left_obj = self.left.evaluate(environ.clone(), state.clone())?;

        if let Object::ReturnVal(ret_val) = &*left_obj.borrow() {
            return Ok(ret_val.unwrap_to_value().clone());
        }

        let right_side = if let Expression::Identifier(identifier) = &*self.right {
            identifier
        } else {
            return Err(PanicObj::new(
                PanicType::IllegalExpression,
                format!(
                    "{} = {} is illegal.",
                    self.to_string(),
                    r_value.borrow().inspect()
                ),
                state.clone(),
            ));
        };

        let name_of_the_identifier = right_side.value.clone();

        let left_side_object = {
            let left_obj_borrow = left_obj.borrow();

            left_obj_borrow.apply_attribute(&name_of_the_identifier, environ, state)?
        };

        *left_side_object.borrow_mut() = r_value.borrow().clone();

        Ok(r_value.clone())
    }
}
