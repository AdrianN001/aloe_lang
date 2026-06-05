use crate::{
    ast::expression::{
        Expression, array_literal::ArrayLiteral, boolean::Boolean, call_expression::CallExpression,
        hash_map_literal::HashMapLiteral, index_expression::IndexExpression,
        infix::InfixExpression, member::MemberExpression,
        scope_resolution::ScopeResolutionExpression,
        value_assign_expression::ValueAssignExpression,
    },
    frame::{
        Frame,
        block_frame::BlockFrame,
        expr_frame::{EvaluationResult, ExpressionFrame},
        state::ExpressionState,
    },
    object::{
        Object,
        error::panic_type::PanicType,
        new_objectref,
        panic_obj::{PanicObj, RuntimeSignal},
        stack_environment::EnvRef,
        state::StateRef,
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

                {
                    interpreter_state
                        .borrow_mut()
                        .set_current_line(call_expr.token.line_number);
                }

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

                {
                    interpreter_state
                        .borrow_mut()
                        .set_current_line(arr_expr.token.line_number);
                }

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

                {
                    interpreter_state
                        .borrow_mut()
                        .set_current_line(unary_expr.token.line_number);
                }

                if !*ready_to_evaluate {
                    return Ok(ExpressionFrame::build_frame_from_expr(
                        &unary_expr.right,
                        environ.clone(),
                    ));
                }
                let (prefix_operator, line_number) = {
                    let expr = &self.expr;
                    match expr {
                        Expression::Prefix(prefix) => (&prefix.operator, prefix.token.line_number),
                        _ => unreachable!(),
                    }
                };

                let value_clone = value.as_ref().unwrap();
                let value_borrow = value_clone.borrow();
                Ok(EvaluationResult::Done(value_borrow.evaluate_prefix(
                    &prefix_operator,
                    line_number,
                    interpreter_state,
                )?))
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

                {
                    interpreter_state
                        .borrow_mut()
                        .set_current_line(index_expr.token.line_number);
                }

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
                                        token: Token::simple(TokenType::KwTrue, "true", 0),
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

                    Ok(EvaluationResult::Push((
                        Frame::BlockFrame(
                            BlockFrame::new(&current_path.statements, environ.clone()).to_ref(),
                        ),
                        environ.clone(),
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
                {
                    interpreter_state
                        .borrow_mut()
                        .set_current_line(hashmap_expr.token.line_number);
                }
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
                {
                    interpreter_state
                        .borrow_mut()
                        .set_current_line(infix_expr.token.line_number);
                }
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
            ExpressionState::While { value, state } => {
                let while_expr = {
                    match &self.expr {
                        Expression::WhileLoop(while_loop) => while_loop,
                        _ => unreachable!(),
                    }
                };

                {
                    interpreter_state
                        .borrow_mut()
                        .set_current_line(while_expr.token.line_number);
                }

                if while_expr.condition.is_none() {
                    state.is_infinite = true;
                    state.is_head_ready = true;
                }

                if let Some(value_from_block) = value {
                    return Ok(EvaluationResult::Done(value_from_block.clone()));
                }

                if !state.is_head_ready && !state.is_infinite {
                    if let Some(conditional_expression) = &while_expr.condition {
                        return Ok(ExpressionFrame::build_frame_from_expr(
                            &conditional_expression.clone(),
                            environ.clone(),
                        ));
                    }
                } else if state.is_head_ready
                    && (state.is_infinite
                        || state
                            .conditional_value
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .is_truthy())
                {
                    let loop_block =
                        BlockFrame::new(&while_expr.block.statements, environ.clone()).to_ref();
                    {
                        let mut loop_block_borrow = loop_block.borrow_mut();
                        loop_block_borrow.set_loop_context(true);
                    }
                    return Ok(EvaluationResult::Push((
                        Frame::BlockFrame(loop_block),
                        environ.clone(),
                    )));
                }

                return Ok(EvaluationResult::Done(new_objectref(Object::NULL_OBJECT)));
            }
            ExpressionState::For { value, state } => {
                let for_expr = {
                    match &self.expr {
                        Expression::ForLoop(for_loop) => for_loop,
                        _ => unreachable!(),
                    }
                };

                {
                    interpreter_state
                        .borrow_mut()
                        .set_current_line(for_expr.token.line_number);
                }

                if let Some(value_from_break) = value {
                    return Ok(EvaluationResult::Done(value_from_break.clone()));
                }

                if let Some(for_loop_iterator_expr) = &for_expr.iterator
                    && state.provided_object.is_none()
                {
                    state.iteration_variable_name = {
                        match &**for_expr.variable.as_ref().unwrap() {
                            Expression::Identifier(identifier) => Some(identifier.value.clone()),
                            other_expression_type => {
                                return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                                    PanicType::MissingIdentifier,
                                    format!(
                                        "expected identifier for 'for loop', got {}",
                                        other_expression_type.to_string()
                                    )
                                    .as_ref(),
                                    interpreter_state.clone(),
                                )));
                            }
                        }
                    };
                    return Ok(ExpressionFrame::build_frame_from_expr(
                        for_loop_iterator_expr,
                        environ,
                    ));
                } else if for_expr.iterator.is_none() {
                    state.is_infinite = true;
                }

                if state.is_infinite {
                    let loop_block =
                        BlockFrame::new(&for_expr.block.statements, environ.clone()).to_ref();
                    {
                        let mut loop_block_borrow = loop_block.borrow_mut();
                        loop_block_borrow.set_loop_context(true);
                    }
                    return Ok(EvaluationResult::Push((
                        Frame::BlockFrame(loop_block),
                        environ.clone(),
                    )));
                }

                if state.provided_object.is_some() && state.iterator.is_none() {
                    let provided_object = state
                        .provided_object
                        .as_ref()
                        .expect("provided object as expression was already evaluated");

                    state.iterator = match &*provided_object.borrow() {
                        Object::Iterator(iterator) => Some(*iterator.clone()),
                        Object::Array(arr) => Some(arr.build_iterator()),
                        Object::String(str) => Some(str.build_char_iterator()),
                        Object::HashMap(hashmap) => Some(hashmap.build_iterator()),
                        _ => {
                            return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                                PanicType::ObjectNotIterable,
                                "value provided to for loop is not an iterator",
                                interpreter_state.clone(),
                            )));
                        }
                    };
                }

                let iterator = state.iterator.as_mut().expect("iterator exists");

                let iteration_value = match iterator._next() {
                    Some(val) => val,
                    None => return Ok(EvaluationResult::Done(new_objectref(Object::NULL_OBJECT))),
                };

                let for_loop_body = &for_expr.block.statements;
                let block = BlockFrame::new(&for_loop_body, environ.clone()).to_ref();

                {
                    let mut block_borrow = block.borrow_mut();
                    block_borrow.set_loop_context(true);
                    block_borrow.add_new_variable(
                        state
                            .iteration_variable_name
                            .as_ref()
                            .expect("iteration variable already initialized"),
                        iteration_value,
                    );
                }

                return Ok(EvaluationResult::Push((
                    Frame::BlockFrame(block),
                    environ.clone(),
                )));
            }
            ExpressionState::Member { value, state } => {
                if let Some(value_from_member_expr) = value {
                    return Ok(EvaluationResult::Done(value_from_member_expr.clone()));
                }

                let member_expr = {
                    match &self.expr {
                        Expression::Member(member_expr) => member_expr,
                        _ => unreachable!(),
                    }
                };

                {
                    interpreter_state
                        .borrow_mut()
                        .set_current_line(member_expr.token.line_number);
                }

                let left_side = if let Some(left) = &state.left_side {
                    left.clone()
                } else {
                    return Ok(ExpressionFrame::build_frame_from_expr(
                        &member_expr.left,
                        environ.clone(),
                    ));
                };

                MemberExpression::eval_step(
                    left_side,
                    &member_expr.right,
                    environ,
                    interpreter_state,
                    &state.call_buffer,
                )
            }
            ExpressionState::ScopeResolution { value, state } => {
                if let Some(value_from_scope_res) = value {
                    return Ok(EvaluationResult::Done(value_from_scope_res.clone()));
                }

                let scope_res_expr = {
                    match &self.expr {
                        Expression::ScopeResolution(scope_res) => scope_res,
                        _ => unreachable!(),
                    }
                };

                {
                    interpreter_state
                        .borrow_mut()
                        .set_current_line(scope_res_expr.token.line_number);
                }

                let left_side = if let Some(left) = &state.left_side {
                    left.clone()
                } else {
                    return Ok(ExpressionFrame::build_frame_from_expr(
                        &scope_res_expr.left,
                        environ.clone(),
                    ));
                };

                ScopeResolutionExpression::eval_step(
                    left_side,
                    &scope_res_expr.right,
                    environ,
                    interpreter_state,
                    &state.call_buffer,
                )
            }
            ExpressionState::ValueAssign { state } => {
                let value_assign_expr = {
                    match &self.expr {
                        Expression::ValueAssign(value_assign) => value_assign,
                        _ => unreachable!(),
                    }
                };

                {
                    interpreter_state
                        .borrow_mut()
                        .set_current_line(value_assign_expr.token.line_number);
                }

                if state.right_value.is_none() {
                    return Ok(ExpressionFrame::build_frame_from_expr(
                        &value_assign_expr.right,
                        environ,
                    ));
                }

                let right_value = state.right_value.as_ref().expect("already initialized");

                ValueAssignExpression::eval_step(
                    &value_assign_expr.left,
                    right_value.clone(),
                    environ,
                    interpreter_state,
                    state,
                )
            }
        }
    }
}
