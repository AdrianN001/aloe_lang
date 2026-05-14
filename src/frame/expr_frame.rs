use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::{Expression, await_expression::AwaitExpression},
    frame::{
        Frame,
        state::{
            ExpressionState, array_state::ArrayState, await_state::AwaitState,
            call_state::CallState, for_state::ForState, hashmap_state::HashMapState,
            if_state::IfState, index_state::IndexState, infix_state::InfixState,
            member_state::MemberState, value_assign_state::ValueAssignState,
            while_state::WhileState,
        },
    },
    object::{
        Object, ObjectRef, panic_obj::RuntimeSignal, stack_environment::EnvRef, state::StateRef,
    },
};

pub type ExprFrameRef = Rc<RefCell<ExpressionFrame>>;

#[derive(Debug)]
pub struct ExpressionFrame {
    pub expr: Expression,
    pub state: ExpressionState,
}

impl ExpressionFrame {
    pub fn new_await_frame(expr: Expression) -> Self {
        Self {
            expr,
            state: ExpressionState::Await {
                future: None,
                state: AwaitState::Start,
            },
        }
    }

    pub fn new_primitive(expr: Expression) -> Self {
        Self {
            expr,
            state: ExpressionState::Primitive,
        }
    }

    pub fn new_functioncall_frame(expr: Expression) -> Self {
        let params_n = {
            match &expr {
                Expression::Call(call_expr) => call_expr.arguments.len(),
                _ => todo!(),
            }
        };

        Self {
            expr,
            state: ExpressionState::Call {
                state: CallState {
                    parameters: vec![],
                    parameters_required_by_func: params_n,
                    current_argument: 0,
                },
                ready_to_evaluate: false,
            },
        }
    }

    pub fn new_array_frame(expr: Expression) -> Self {
        let elements_n = {
            match &expr {
                Expression::Array(arr_expr) => arr_expr.elements.len(),
                _ => todo!(),
            }
        };

        Self {
            expr,
            state: ExpressionState::Array {
                ready_to_evaluate: false,
                state: ArrayState {
                    elements: vec![],
                    curr_element: 0,
                    number_of_elements: elements_n,
                },
            },
        }
    }

    pub fn new_infix_frame(expr: Expression) -> Self {
        Self {
            expr,
            state: ExpressionState::Infix {
                ready_to_evaluate: false,
                state: InfixState {
                    left: None,
                    right: None,
                },
            },
        }
    }

    pub fn new_index_frame(expr: Expression) -> Self {
        Self {
            expr,
            state: ExpressionState::Index {
                ready_to_evaluate: false,
                state: IndexState {
                    indexable: None,
                    index: None,
                },
            },
        }
    }

    pub fn new_unary_frame(expr: Expression) -> Self {
        Self {
            expr,
            state: ExpressionState::Unary {
                value: None,
                ready_to_evaluate: false,
            },
        }
    }

    pub fn new_if_frame(expr: Expression, environ: EnvRef) -> Self {
        Self {
            expr,
            state: ExpressionState::If {
                value: None,
                state: IfState::new(environ),
            },
        }
    }

    pub fn new_hashmap_frame(expr: Expression) -> Self {
        Self {
            expr,
            state: ExpressionState::HashMap {
                ready_to_evaluate: false,
                state: HashMapState {
                    current_element: 0,
                    keys: Vec::new(),
                    values: Vec::new(),
                },
            },
        }
    }

    pub fn new_while_frame(expr: Expression) -> Self {
        Self {
            expr,
            state: ExpressionState::While {
                value: None,
                state: WhileState {
                    is_infinite: false,
                    conditional_value: None,
                    is_head_ready: false,
                },
            },
        }
    }

    pub fn new_for_frame(expr: Expression) -> Self {
        Self {
            expr,
            state: ExpressionState::For {
                value: None,
                state: ForState {
                    is_infinite: false,
                    provided_object: None,
                    iterator: None,
                    iteration_variable_name: None,
                },
            },
        }
    }

    pub fn new_member_frame(expr: Expression) -> Self {
        Self {
            expr,
            state: ExpressionState::Member {
                value: None,
                state: MemberState {
                    left_side: None,
                    call_buffer: vec![],
                },
            },
        }
    }

    pub fn new_value_assign_frame(expr: Expression) -> Self {
        Self {
            expr,
            state: ExpressionState::ValueAssign {
                state: ValueAssignState {
                    left_value: None,
                    right_value: None,
                },
            },
        }
    }

    pub fn build_frame_from_expr(expression: &Expression, environ: EnvRef) -> EvaluationResult {
        let new_frame = match expression {
            Expression::AwaitExpr(_) => {
                ExpressionFrame::new_await_frame(expression.clone()).to_ref()
            }
            Expression::Array(_) => ExpressionFrame::new_array_frame(expression.clone()).to_ref(),
            Expression::Call(_) => {
                ExpressionFrame::new_functioncall_frame(expression.clone()).to_ref()
            }
            Expression::Index(_) => ExpressionFrame::new_index_frame(expression.clone()).to_ref(),
            Expression::IntegerLiteral(_)
            | Expression::Bool(_)
            | Expression::FloatLiteral(_)
            | Expression::String(_)
            | Expression::Identifier(_)
            | Expression::Function(_)
            | Expression::AsyncFunction(_) => {
                ExpressionFrame::new_primitive(expression.clone()).to_ref()
            }
            Expression::Prefix(_) => ExpressionFrame::new_unary_frame(expression.clone()).to_ref(),
            Expression::If(_) => {
                ExpressionFrame::new_if_frame(expression.clone(), environ.clone()).to_ref()
            }
            Expression::HashMapLiteral(_) => {
                ExpressionFrame::new_hashmap_frame(expression.clone()).to_ref()
            }
            Expression::Infix(_) => ExpressionFrame::new_infix_frame(expression.clone()).to_ref(),
            Expression::WhileLoop(_) => {
                ExpressionFrame::new_while_frame(expression.clone()).to_ref()
            }
            Expression::ForLoop(_) => ExpressionFrame::new_for_frame(expression.clone()).to_ref(),
            Expression::Member(_) => ExpressionFrame::new_member_frame(expression.clone()).to_ref(),
            Expression::ValueAssign(_) => {
                ExpressionFrame::new_value_assign_frame(expression.clone()).to_ref()
            }
            Expression::InvalidExpression => unreachable!(),
        };

        EvaluationResult::Push((Frame::ExpressionFrame(new_frame), environ))
    }
    pub fn to_ref(self) -> ExprFrameRef {
        Rc::new(RefCell::new(self))
    }
}

impl ExpressionFrame {
    pub fn resume_with(
        &mut self,
        object: ObjectRef,
        interpreter_state: StateRef,
    ) -> Result<(), RuntimeSignal> {
        match &mut self.state {
            ExpressionState::Call {
                ready_to_evaluate,
                state,
            } => {
                state.parameters.push(object);

                let params_needed = state.parameters_required_by_func;
                let params_already_resolved = &state.parameters;
                if params_needed == params_already_resolved.len() {
                    *ready_to_evaluate = true;
                }
                Ok(())
            }
            ExpressionState::Await { future, state } => {
                let awaited_value =
                    AwaitExpression::eval2(&self.expr, object.clone(), interpreter_state)?;
                *future = Some(awaited_value.clone());
                *state = AwaitState::Waiting;
                Ok(())
            }
            ExpressionState::Unary {
                value,
                ready_to_evaluate,
            } => {
                *value = Some(object.clone());
                *ready_to_evaluate = true;
                Ok(())
            }
            ExpressionState::Array {
                ready_to_evaluate,
                state,
            } => {
                state.elements.push(object.clone());

                if state.elements.len() == state.number_of_elements {
                    *ready_to_evaluate = true;
                }

                Ok(())
            }
            ExpressionState::Index {
                ready_to_evaluate,
                state,
            } => {
                if state.indexable.is_none() {
                    state.indexable = Some(object);
                } else {
                    state.index = Some(object);

                    *ready_to_evaluate = true;
                }
                Ok(())
            }
            ExpressionState::If { value, state } => {
                if !state.path_found {
                    let is_object_truthy = {
                        let object_borrow = object.borrow();
                        object_borrow.is_truthy()
                    };

                    if is_object_truthy {
                        state.path_found = true;
                    } else {
                        state.current_path += 1;
                    }
                } else {
                    *value = Some(object.clone())
                }
                Ok(())
            }
            ExpressionState::HashMap {
                ready_to_evaluate,
                state,
            } => {
                let hashmap_expr = {
                    match &self.expr {
                        Expression::HashMapLiteral(hashmap) => hashmap,
                        _ => unreachable!(),
                    }
                };

                if state.current_element % 2 == 0 {
                    state.keys.push(object.clone());
                } else {
                    state.values.push(object.clone());
                }

                if state.values.len() >= hashmap_expr.pairs.len() {
                    *ready_to_evaluate = true;
                } else {
                    state.current_element += 1;
                }

                Ok(())
            }
            ExpressionState::Infix {
                ready_to_evaluate,
                state,
            } => {
                if state.left.is_none() {
                    state.left = Some(object.clone());
                } else if state.right.is_none() {
                    state.right = Some(object.clone());
                    *ready_to_evaluate = true;
                }
                Ok(())
            }

            ExpressionState::While { value, state } => {
                if !state.is_head_ready {
                    state.conditional_value = Some(object.clone());
                    state.is_head_ready = true;
                } else {
                    let is_break_value = {
                        let object_borrow = object.borrow();
                        matches!(*object_borrow, Object::BreakVal(_))
                    };
                    if is_break_value {
                        if let Object::BreakVal(break_val) = &*object.borrow() {
                            *value = Some(*break_val.value.clone());
                        }
                    }
                    state.conditional_value = None;

                    if !state.is_infinite {
                        state.is_head_ready = false;
                    }
                }

                Ok(())
            }
            ExpressionState::For { value, state } => {
                if state.provided_object.is_none() && !state.is_infinite {
                    state.provided_object = Some(object.clone());
                } else if state.is_infinite || state.provided_object.is_some() {
                    let is_break_value = {
                        let object_borrow = object.borrow();
                        matches!(*object_borrow, Object::BreakVal(_))
                    };
                    if is_break_value {
                        if let Object::BreakVal(break_val) = &*object.borrow() {
                            *value = Some(*break_val.value.clone());
                        }
                    }
                }

                Ok(())
            }
            ExpressionState::Member { value, state } => {
                if state.left_side.is_none() {
                    state.left_side = Some(object.clone());
                    return Ok(());
                }

                let member_expr = {
                    match &self.expr {
                        Expression::Member(member_expr) => member_expr,
                        _ => unreachable!(),
                    }
                };

                if let Expression::Call(call_expr) = &*member_expr.right {
                    let expected_arg_n = call_expr.arguments.len();

                    if expected_arg_n != state.call_buffer.len() {
                        state.call_buffer.push(object.clone());
                        return Ok(());
                    } else {
                        *value = Some(object.clone());
                    }
                }

                Ok(())
            }
            ExpressionState::ValueAssign { state } => {
                if state.right_value.is_none() {
                    state.right_value = Some(object.clone());
                } else if state.left_value.is_none() {
                    state.left_value = Some(object.clone());
                }

                Ok(())
            }

            ExpressionState::Primitive => Ok(()),
        }
    }
}

#[derive(Debug)]
pub enum EvaluationResult {
    Done(ObjectRef),
    Pending,
    Return(ObjectRef),

    Continue,
    Break(ObjectRef),

    Push((Frame, EnvRef)),
}
