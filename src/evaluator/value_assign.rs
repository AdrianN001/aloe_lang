use crate::{
    ast::expression::{
        Expression, array_literal::ArrayLiteral, value_assign_expression::ValueAssignExpression,
    },
    frame::{
        expr_frame::{EvaluationResult, ExpressionFrame},
        state::value_assign_state::ValueAssignState,
    },
    object::{
        Object, ObjectRef,
        array::Array,
        error::panic_type::PanicType,
        new_objectref,
        panic_obj::{PanicObj, RuntimeSignal},
        stack_environment::EnvRef,
        state::StateRef,
    },
};

impl ValueAssignExpression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        let right = self.right.evaluate(environ.clone(), state.clone())?;

        match &*self.left {
            Expression::Identifier(identifier) => {
                let mut environ_borrow = environ.borrow_mut();
                if !environ_borrow.try_to_assign(&identifier.value, right.clone()) {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::VariableIsNotDeclared,
                        format!("variable '{}' was not declared.", &identifier.value),
                        state.clone(),
                    )));
                }
                Ok(right.clone())
            }
            Expression::Array(destruct_arr) => {
                let identifiers = ValueAssignExpression::get_identifier_from_destruct_arr(
                    destruct_arr,
                    state.clone(),
                )?;

                ValueAssignExpression::evaluate_destructuring(&identifiers, right, environ, state)
            }
            Expression::Index(index_expr) => {
                index_expr.evaluate_value_assign(environ.clone(), right.clone(), state.clone())?;
                Ok(right.clone())
            }
            Expression::Member(member_expr) => {
                member_expr.evaluate_value_assign(environ.clone(), state.clone(), right.clone())
            }
            other_expression => Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::UnexpectedRValue,
                format!("expected LValue, got {}", other_expression.to_string()),
                state.clone(),
            ))),
        }
    }

    pub fn evaluate_with_let_binding(
        &self,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        let right = self.right.evaluate(environ.clone(), state.clone())?;

        match &*self.left {
            Expression::Array(destruct_arr) => {
                let identifiers = ValueAssignExpression::get_identifier_from_destruct_arr(
                    destruct_arr,
                    state.clone(),
                )?;

                ValueAssignExpression::evaluate_destructuring_with_let_binding(
                    &identifiers,
                    right,
                    environ,
                    state,
                )
            }
            Expression::Identifier(identifier) => {
                environ
                    .borrow_mut()
                    .set_to_lowest_level(&identifier.value, right.clone());
                Ok(right)
            }
            other_expression => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::WrongSyntax,
                    format!(
                        "expcted after let keyword an identifier, or an array for destructuring, got: '{}'",
                        other_expression.to_string()
                    ),
                    state,
                )));
            }
        }
    }

    pub fn get_identifier_from_destruct_arr(
        destruct_arr: &ArrayLiteral,
        state: StateRef,
    ) -> Result<Vec<String>, RuntimeSignal> {
        if destruct_arr.elements.is_empty() {
            return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                PanicType::WrongSyntax,
                "destructuring array must not be empty",
                state,
            )));
        }
        destruct_arr
            .elements
            .iter()
            .map(|expr| match expr {
                Expression::Identifier(identifier) => Ok(identifier.value.clone()),
                other_type => {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::WrongSyntax,
                        format!(
                            "expected only identifiers in destructuring array, but got: '{}'",
                            other_type.to_string()
                        ),
                        state.clone(),
                    )));
                }
            })
            .collect()
    }

    pub fn evaluate_destructuring_with_let_binding(
        identifiers: &[String],
        right_side_value: ObjectRef,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        let right_side_borrow = right_side_value.borrow();

        let array = match &*right_side_borrow {
            Object::Array(arr) => arr,
            other_type => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::Destructuring,
                    format!("{} can not be destructred", other_type.get_type()),
                    state,
                )));
            }
        };

        let array_items = array.items.clone();
        let array_length = array.items.len();

        if identifiers.len() > array_length {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::Destructuring,
                format!(
                    "Array on the right side has a length of {}, but there are {} identifiers the left side.",
                    array_length,
                    identifiers.len()
                ),
                state,
            )));
        } else if identifiers.len() == array_length {
            identifiers
                .iter()
                .zip(array_items)
                .for_each(|(identifier, value)| {
                    environ
                        .clone()
                        .borrow_mut()
                        .set_to_lowest_level(identifier, value);
                });
            Ok(right_side_value.clone())
        } else {
            for indx in 0..identifiers.len() - 1 {
                let identifier = &identifiers[indx];
                let value = &array_items[indx];

                environ
                    .borrow_mut()
                    .set_to_lowest_level(&identifier, value.clone());
            }

            let last_identifer = identifiers.last().expect("last exists");
            let sub_array = &array_items[identifiers.len() - 1..];
            let rest_arr = new_objectref(Object::Array(Box::new(Array {
                items: sub_array.to_vec(),
            })));

            environ
                .borrow_mut()
                .set_to_lowest_level(last_identifer, rest_arr);

            Ok(right_side_value.clone())
        }
    }

    fn evaluate_destructuring(
        identifiers: &[String],
        right_side_value: ObjectRef,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        let right_side_borrow = right_side_value.borrow();

        let array = match &*right_side_borrow {
            Object::Array(arr) => arr,
            other_type => {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::Destructuring,
                    format!("{} can not be destructred", other_type.get_type()),
                    state,
                )));
            }
        };

        let array_items = array.items.clone();
        let array_length = array.items.len();

        if identifiers.len() > array_length {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::Destructuring,
                format!(
                    "Array on the right side has a length of {}, but there are {} identifiers the left side.",
                    array_length,
                    identifiers.len()
                ),
                state,
            )));
        } else if identifiers.len() == array_length {
            identifiers
                .iter()
                .zip(array_items)
                .map(|(identifier, value)| {
                    let variable_found = environ.borrow_mut().try_to_assign(identifier, value);
                    if !variable_found {
                        return Err(RuntimeSignal::Panic(PanicObj::new(
                            PanicType::VariableIsNotDeclared,
                            format!("variable '{}' was not declared.", identifier),
                            state.clone(),
                        )));
                    }
                    Ok(())
                })
                .collect::<Result<Vec<()>, RuntimeSignal>>()?;
            Ok(right_side_value.clone())
        } else {
            for indx in 0..identifiers.len() - 1 {
                let identifier = &identifiers[indx];
                let value = &array_items[indx];

                let variable_found = environ
                    .borrow_mut()
                    .try_to_assign(identifier, value.clone());
                if !variable_found {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::VariableIsNotDeclared,
                        format!("variable '{}' was not declared.", identifier),
                        state.clone(),
                    )));
                }
            }

            let last_identifer = identifiers.last().expect("last exists");
            let sub_array = &array_items[identifiers.len() - 1..];
            let rest_arr = new_objectref(Object::Array(Box::new(Array {
                items: sub_array.to_vec(),
            })));

            let variable_found = environ.borrow_mut().try_to_assign(last_identifer, rest_arr);
            if !variable_found {
                return Err(RuntimeSignal::Panic(PanicObj::new(
                    PanicType::VariableIsNotDeclared,
                    format!("variable '{}' was not declared.", last_identifer),
                    state.clone(),
                )));
            }

            Ok(right_side_value.clone())
        }
    }

    pub fn eval_step(
        left_expr: &Expression,
        right_value: ObjectRef,
        environ: EnvRef,
        interpreter_state: StateRef,
        state: &ValueAssignState,
    ) -> Result<EvaluationResult, RuntimeSignal> {
        match left_expr {
            Expression::Identifier(identifier) => {
                let mut environ_borrow = environ.borrow_mut();
                if !environ_borrow.try_to_assign(&identifier.value, right_value.clone()) {
                    return Err(RuntimeSignal::Panic(PanicObj::new(
                        PanicType::VariableIsNotDeclared,
                        format!("variable '{}' was not declared.", &identifier.value),
                        interpreter_state.clone(),
                    )));
                }
                Ok(EvaluationResult::Done(right_value.clone()))
            }
            Expression::Index(_) | Expression::Member(_) => {
                if state.left_value.is_none() {
                    return Ok(ExpressionFrame::build_frame_from_expr(left_expr, environ));
                }
                let left_value = state.left_value.as_ref().expect("already initialized");

                if left_value.as_ptr() != right_value.as_ptr() {
                    *left_value.borrow_mut() = right_value.borrow().clone();
                }
                Ok(EvaluationResult::Done(right_value.clone()))
            }
            Expression::Array(array_expr) => {
                let identifiers = ValueAssignExpression::get_identifier_from_destruct_arr(
                    array_expr,
                    interpreter_state.clone(),
                )?;
                let destruct_result = ValueAssignExpression::evaluate_destructuring(
                    &identifiers,
                    right_value,
                    environ,
                    interpreter_state,
                )?;

                Ok(EvaluationResult::Done(destruct_result))
            }
            other_expression => Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::UnexpectedRValue,
                format!("expected LValue, got {}", other_expression.to_string()),
                interpreter_state.clone(),
            ))),
        }
    }
}
