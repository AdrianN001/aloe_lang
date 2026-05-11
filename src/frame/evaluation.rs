use crate::{
    ast::expression::{
        Expression, array_literal::ArrayLiteral, boolean::Boolean, call_expression::CallExpression,
        hash_map_literal::HashMapLiteral, index_expression::IndexExpression,
        infix::InfixExpression,
    },
    frame::{
        Frame,
        block_frame::BlockFrame,
        expr_frame::{EvaluationResult, ExpressionFrame},
        state::ExpressionState,
    },
    object::{
        Object, new_objectref, panic_obj::RuntimeSignal, stack_environment::EnvRef, state::StateRef,
    },
    token::{Token, token_type::TokenType},
};

impl ExpressionFrame {
    pub fn eval_step(
        &mut self,
        environ: EnvRef,
        interpreter_state: StateRef,
    ) -> Result<EvaluationResult, RuntimeSignal> {
        match &mut self.state {
            ExpressionState::Await { future, state } => {
                state.eval_step(&self.expr, future, environ, interpreter_state)
            }
            ExpressionState::Call {
                state,
                ready_to_evaluate,
            } => {
                let call_expr = {
                    match &self.expr {
                        Expression::Call(call_expr) => call_expr,
                        _ => todo!(),
                    }
                };

                if !*ready_to_evaluate && state.parameters_required_by_func != 0 {
                    state.current_argument += 1;
                    return Ok(ExpressionFrame::build_frame_from_expr(
                        &call_expr.arguments[state.current_argument - 1],
                        environ,
                    ));
                }
                let (callable_object, questionmark_set, bang_set) = {
                    let callable_object = call_expr
                        .function
                        .evaluate(environ.clone(), interpreter_state.clone())?;
                    let questionmark_set = call_expr.question_mark_set;
                    let bang_set = call_expr.bang_set;

                    (callable_object, questionmark_set, bang_set)
                };
                let args = &state.parameters;

                let function_name = {
                    let raw_name = match &*call_expr.function {
                        Expression::Identifier(identifier) => identifier.value.clone(),
                        _ => "anonymm function".to_string(),
                    };
                    raw_name
                };
                let return_value = CallExpression::call_with_arguments(
                    callable_object,
                    &args,
                    function_name,
                    interpreter_state,
                    environ,
                    questionmark_set,
                    bang_set,
                )?;
                Ok(EvaluationResult::Done(return_value))
            }

            ExpressionState::Array {
                ready_to_evaluate,
                state,
            } => {
                let number_of_elements = state.number_of_elements;

                let arr_expr = {
                    match &self.expr {
                        Expression::Array(arr) => arr,
                        _ => unreachable!(),
                    }
                };

                if !*ready_to_evaluate && number_of_elements != 0 {
                    state.curr_element += 1;
                    return Ok(ExpressionFrame::build_frame_from_expr(
                        &arr_expr.elements[state.curr_element - 1],
                        environ.clone(),
                    ));
                }
                let elements = &state.elements;
                Ok(EvaluationResult::Done(ArrayLiteral::eval_step(
                    &elements,
                    interpreter_state.clone(),
                )?))
            }

            ExpressionState::Unary {
                value,
                ready_to_evaluate,
            } => {
                let unary_expr = {
                    match &self.expr {
                        Expression::Prefix(prefix_expr) => prefix_expr,
                        _ => unreachable!(),
                    }
                };

                if !*ready_to_evaluate {
                    return Ok(ExpressionFrame::build_frame_from_expr(
                        &unary_expr.right,
                        environ.clone(),
                    ));
                }
                let prefix_operator = {
                    let expr = &self.expr;
                    match expr {
                        Expression::Prefix(prefix) => &prefix.operator,
                        _ => unreachable!(),
                    }
                };

                let value_clone = value.as_ref().unwrap();
                let value_borrow = value_clone.borrow();
                Ok(EvaluationResult::Done(
                    value_borrow.evaluate_prefix(&prefix_operator, interpreter_state)?,
                ))
            }

            ExpressionState::Primitive => match self.expr.evaluate(environ, interpreter_state) {
                Ok(ok_value) => Ok(EvaluationResult::Done(ok_value)),
                Err(e) => Err(e),
            },
            ExpressionState::Index {
                ready_to_evaluate,
                state,
            } => {
                let index_expr = {
                    match &self.expr {
                        Expression::Index(indx) => indx,
                        _ => unreachable!(),
                    }
                };

                if !*ready_to_evaluate {
                    if state.indexable.is_none() {
                        return Ok(ExpressionFrame::build_frame_from_expr(
                            &*index_expr.left,
                            environ,
                        ));
                    } else {
                        return Ok(ExpressionFrame::build_frame_from_expr(
                            &*index_expr.right,
                            environ,
                        ));
                    }
                }

                let left = state.indexable.as_ref().unwrap();
                let right = state.index.as_ref().unwrap();
                Ok(EvaluationResult::Done(IndexExpression::eval_step(
                    left.clone(),
                    right.clone(),
                    interpreter_state.clone(),
                )?))
            }
            ExpressionState::If { value, state } => {
                let if_expr = {
                    match &self.expr {
                        Expression::If(if_expr) => if_expr,
                        _ => unreachable!(),
                    }
                };

                if let Some(value_from_block) = value {
                    return Ok(EvaluationResult::Done(value_from_block.clone()));
                }

                if !state.path_found {
                    let current_condition_object = match &state.current_path {
                        0 => if_expr.condition.clone(),
                        alternative_path_index
                            if !if_expr.alternatives.is_empty()
                                && *alternative_path_index - 1 < if_expr.alternatives.len() =>
                        {
                            if_expr.alternatives[*alternative_path_index - 1].0.clone()
                        }
                        _ => {
                            if if_expr.else_block.is_none() {
                                return Ok(EvaluationResult::Done(new_objectref(
                                    Object::NULL_OBJECT,
                                )));
                            } else {
                                // handle else block als last elif (true) block
                                return Ok(ExpressionFrame::build_frame_from_expr(
                                    &Expression::Bool(Boolean {
                                        value: true,
                                        token: Token::simple(TokenType::KwTrue, "true"),
                                    }),
                                    environ,
                                ));
                            }
                        }
                    };
                    Ok(ExpressionFrame::build_frame_from_expr(
                        &current_condition_object,
                        environ,
                    ))
                } else {
                    let current_path = match &state.current_path {
                        0 => if_expr.consequence.clone(),
                        alternative_path_index
                            if !if_expr.alternatives.is_empty()
                                && *alternative_path_index - 1 < if_expr.alternatives.len() =>
                        {
                            if_expr.alternatives[*alternative_path_index - 1].1.clone()
                        }
                        _ => {
                            if let Some(else_block) = &if_expr.else_block {
                                else_block.clone()
                            } else {
                                unreachable!()
                            }
                        }
                    };

                    Ok(EvaluationResult::Push(Frame::BlockFrame(
                        BlockFrame::new(&current_path.statements, environ).to_ref(),
                    )))
                }
            }
            ExpressionState::HashMap {
                ready_to_evaluate,
                state,
            } => {
                let hashmap_expr = {
                    match &self.expr {
                        Expression::HashMapLiteral(hashmap_literal) => hashmap_literal,
                        _ => unreachable!(),
                    }
                };
                if *ready_to_evaluate || hashmap_expr.pairs.is_empty() {
                    return Ok(EvaluationResult::Done(
                        HashMapLiteral::evaluate_with_evaluated_vals(
                            &state.keys,
                            &state.values,
                            interpreter_state,
                        )?,
                    ));
                }

                //TODO: optimalization
                let current_item = state.current_element;
                let current_expression = if current_item % 2 == 0 {
                    // key
                    let current_key = current_item / 2;
                    let expression = hashmap_expr.pairs.iter().nth(current_key).unwrap().0;
                    expression
                } else {
                    // value
                    let current_value = (current_item - 1) / 2;
                    let expression = hashmap_expr.pairs.iter().nth(current_value).unwrap().1;
                    expression
                };

                Ok(ExpressionFrame::build_frame_from_expr(
                    current_expression,
                    environ,
                ))
            }

            ExpressionState::Infix {
                ready_to_evaluate,
                state,
            } => {
                let infix_expr = {
                    match &self.expr {
                        Expression::Infix(infix) => infix,
                        _ => unreachable!(),
                    }
                };
                if !*ready_to_evaluate {
                    if state.left.is_none() {
                        return Ok(ExpressionFrame::build_frame_from_expr(
                            &infix_expr.left,
                            environ.clone(),
                        ));
                    } else if state.right.is_none() {
                        return Ok(ExpressionFrame::build_frame_from_expr(
                            &infix_expr.right,
                            environ.clone(),
                        ));
                    }
                    unreachable!();
                } else {
                    let left = state.left.as_ref().unwrap().clone();
                    let right = state.right.as_ref().unwrap().clone();
                    let operator = infix_expr.operator.clone();
                    Ok(EvaluationResult::Done(InfixExpression::evaluate_step(
                        left,
                        right,
                        operator,
                        interpreter_state.clone(),
                    )?))
                }
            }
        }
    }
}
