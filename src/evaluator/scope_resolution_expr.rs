use crate::{
    ast::expression::{
        Expression, member::MemberExpression, scope_resolution::ScopeResolutionExpression,
    },
    frame::expr_frame::{EvaluationResult, ExpressionFrame},
    object::{
        Object, ObjectRef,
        enum_model::EnumModel,
        error::panic_type::PanicType,
        integer::Integer,
        new_objectref,
        panic_obj::{PanicObj, RuntimeSignal},
        stack_environment::EnvRef,
        state::StateRef,
    },
};

impl ScopeResolutionExpression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        {
            state.borrow_mut().set_current_line(self.token.line_number);
        }
        let left_obj = self.left.evaluate(environ.clone(), state.clone())?;
        let left_obj_borrow = left_obj.borrow();

        let module_obj = match &*left_obj_borrow {
            Object::Module(module) => module,
            Object::EnumModel(enum_model) => {
                return self.evaluate_with_enum_object(&enum_model, state);
            }
            other_type => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::OperatorIsNotSupported,
                    format!(
                        "the scope resolution operator (::) ist not supported on: {}",
                        other_type.get_type()
                    ),
                    state,
                )));
            }
        };

        match &*self.right {
            Expression::Call(call_expr) => {
                let name_of_function =
                    MemberExpression::get_call_expressions_identifier(call_expr, state.clone())?;
                let args = call_expr.evaluate_arguments(environ.clone(), state.clone())?;

                let function_call_result =
                    module_obj.search_function(&name_of_function, &args, environ, state.clone())?;

                let return_value_cloned = function_call_result.clone();

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
                        return Err(RuntimeSignal::Propagation(function_call_result.clone()));
                    }
                }

                Ok(function_call_result)
            }
            Expression::Identifier(identifier) => {
                let name_of_variable = &identifier.value;

                module_obj.search_variable(name_of_variable, state)
            }
            other_expr_type => Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::IllegalExpression,
                format!(
                    "'{}::{}' is illegal.",
                    left_obj_borrow.get_type(),
                    other_expr_type.to_string()
                ),
                state.clone(),
            ))),
        }
    }

    pub fn eval_step(
        left_side: ObjectRef,
        right_expression: &Expression,
        environ: EnvRef,
        state: StateRef,
        awaited_arguments: &[ObjectRef],
    ) -> Result<EvaluationResult, RuntimeSignal> {
        let left_side_borrow = left_side.borrow();
        let module_object = match &*left_side_borrow {
            Object::Module(module) => module,
            Object::EnumModel(enum_model) => {
                return ScopeResolutionExpression::evaluate_step_with_enum_object(
                    &*enum_model,
                    right_expression,
                    state,
                );
            }
            other_type => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::OperatorIsNotSupported,
                    format!(
                        "the scope resolution operator (::) ist not supported on: {}",
                        other_type.get_type()
                    ),
                    state,
                )));
            }
        };

        match right_expression {
            Expression::Identifier(identifier) => {
                let name_of_variable = &identifier.value;

                Ok(EvaluationResult::Done(
                    module_object.search_variable(name_of_variable, state)?,
                ))
            }
            Expression::Call(call_expression) => {
                let name_of_method = MemberExpression::get_call_expressions_identifier(
                    call_expression,
                    state.clone(),
                )?;

                if call_expression.arguments.len() != awaited_arguments.len() {
                    return Ok(ExpressionFrame::build_frame_from_expr(
                        &call_expression.arguments[awaited_arguments.len()],
                        environ.clone(),
                    ));
                }

                let args = awaited_arguments;
                let function_call_result =
                    module_object.search_function(&name_of_method, args, environ, state.clone())?;

                let return_value_cloned = function_call_result.clone();

                if let Object::Err(err) = &*return_value_cloned.borrow() {
                    if call_expression.question_mark_set && !state.borrow().is_function_context() {
                        return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                            PanicType::PropagationFromNonfunctionalContext,
                            "tried to use ? on a function, without function-context",
                            state.clone(),
                        )));
                    }
                    if call_expression.bang_set {
                        return Err(RuntimeSignal::Panic(PanicObj::from_error(
                            err,
                            state.clone(),
                        )));
                    } else if call_expression.question_mark_set {
                        return Err(RuntimeSignal::Propagation(function_call_result.clone()));
                    }
                }

                Ok(EvaluationResult::Done(function_call_result))
            }
            other_expr_type => Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::IllegalExpression,
                format!(
                    "'{}::{}' is illegal.",
                    left_side_borrow.get_type(),
                    other_expr_type.to_string()
                ),
                state.clone(),
            ))),
        }
    }

    fn evaluate_with_enum_object(
        &self,
        enum_obj: &EnumModel,
        state: StateRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        let right_side_identifier_value = match &*self.right {
            Expression::Identifier(identifier) => identifier.value.clone(),
            other_expression => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::WrongSyntax,
                    format!(
                        "expected identifier on the right side of the scope resolution operator with enum, got: {}",
                        other_expression.to_string()
                    ),
                    state,
                )));
            }
        };

        let value = match enum_obj.values.get(&right_side_identifier_value) {
            Some(val) => val,
            None => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::UnknownEnumValue,
                    format!(
                        "{}::{} does not exist",
                        enum_obj.name, right_side_identifier_value
                    ),
                    state,
                )));
            }
        };

        Ok(new_objectref(Object::Int(Integer {
            value: *value as i64,
        })))
    }
    fn evaluate_step_with_enum_object(
        enum_obj: &EnumModel,
        right_expression: &Expression,
        state: StateRef,
    ) -> Result<EvaluationResult, RuntimeSignal> {
        let right_side_identifier_value = match right_expression {
            Expression::Identifier(identifier) => identifier.value.clone(),
            other_expression => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::WrongSyntax,
                    format!(
                        "expected identifier on the right side of the scope resolution operator with enum, got: {}",
                        other_expression.to_string()
                    ),
                    state,
                )));
            }
        };

        let value = match enum_obj.values.get(&right_side_identifier_value) {
            Some(val) => val,
            None => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::UnknownEnumValue,
                    format!(
                        "{}::{} does not exist",
                        enum_obj.name, right_side_identifier_value
                    ),
                    state,
                )));
            }
        };

        Ok(EvaluationResult::Done(new_objectref(Object::Int(
            Integer {
                value: *value as i64,
            },
        ))))
    }
}
