use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::{Expression, call_expression::CallExpression, member::MemberExpression},
    object::{
        Object, ObjectRef,
        error::panic_type::PanicType,
        panic_obj::{PanicObj, RuntimeSignal},
        return_value::ReturnValue,
        stack_environment::EnvRef,
        state::StateRef,
        struct_object::StructObject,
    },
};

impl MemberExpression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
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
                        call_expr.bang_set,
                        call_expr.question_mark_set,
                    );
                }

                let mut obj = left_obj.borrow_mut();

                let return_value =
                    obj.apply_method(&name_of_method, &args, environ.clone(), state.clone())?;

                let return_value_cloned = return_value.clone();

                if let Object::Err(err) = &*return_value_cloned.borrow() {
                    if call_expr.question_mark_set && !state.borrow().is_function_context() {
                        return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                            PanicType::PropagationFromNonfunctionalContext,
                            "tried to use ? on a function, without function-context",
                            state.clone(),
                        )));
                    }
                    if call_expr.bang_set {
                        return Err(RuntimeSignal::Panic(PanicObj::from_error(
                            err,
                            state.clone(),
                        )));
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
            other_expr_type => Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::IllegalExpression,
                format!(
                    "'{}.{}' is illegal.",
                    left_obj.borrow().inspect(),
                    other_expr_type.to_string()
                ),
                state.clone(),
            ))),
        }
    }

    fn check_early_for_struct_method_call(left_obj: ObjectRef) -> bool {
        let left_obj_borrow = left_obj.borrow();

        matches!(&*left_obj_borrow, Object::StructObject(_))
    }

    fn get_call_expressions_identifier(
        call_expr: &CallExpression,
        state: StateRef,
    ) -> Result<String, RuntimeSignal> {
        match &*call_expr.function {
            Expression::Identifier(identifier) => Ok(identifier.value.clone()),
            _ => Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::IllegalExpression,
                format!("'{}' is illegal", call_expr.to_string()),
                state.clone(),
            ))),
        }
    }

    pub fn evaluate_value_assign(
        &self,
        environ: EnvRef,
        state: StateRef,
        r_value: ObjectRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        let left_obj = self.left.evaluate(environ.clone(), state.clone())?;
        let mut left_obj_borrow = left_obj.borrow_mut();

        if let Object::ReturnVal(ret_val) = &*left_obj_borrow {
            return Ok(ret_val.unwrap_to_value().clone());
        }

        match &*self.right {
            Expression::Identifier(attribute) => {
                match &mut *left_obj_borrow {
                    Object::StructObject(struct_object) => {
                        struct_object.set_attribute(&attribute.value, r_value.clone());
                    }
                    other_type => {
                        return Err(RuntimeSignal::Panic(PanicObj::new(
                            PanicType::IllegalExpression,
                            format!(
                                "cannot overwrite the attribute of a native ({}) object.",
                                other_type.get_type()
                            ),
                            state,
                        )));
                    }
                };
            }
            Expression::Call(method_call) => {
                match &mut *left_obj_borrow {
                    Object::StructObject(_) => {
                        let args =
                            method_call.evaluate_arguments(environ.clone(), state.clone())?;
                        let method_name =
                            Self::get_call_expressions_identifier(method_call, state.clone())?;

                        let method_return_value = StructObject::apply_method(
                            &method_name,
                            left_obj.clone(),
                            &args,
                            environ.clone(),
                            state,
                            method_call.bang_set,
                            method_call.question_mark_set,
                        )?;

                        *method_return_value.borrow_mut() = r_value.borrow().clone();
                    }
                    other_type => {
                        return Err(RuntimeSignal::Panic(PanicObj::new(
                            PanicType::IllegalExpression,
                            format!(
                                "cannot overwrite the attribute of a native ({}) object.",
                                other_type.get_type()
                            ),
                            state,
                        )));
                    }
                };
            }
            _ => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::IllegalExpression,
                    format!(
                        "{} = {} is illegal.",
                        self.to_string(),
                        r_value.borrow().inspect()
                    ),
                    state.clone(),
                )));
            }
        };

        Ok(r_value.clone())
    }
}
