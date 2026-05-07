use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::{Expression, await_expression::AwaitExpression},
    frame::state::{
        ExpressionState, array_state::ArrayState, await_state::AwaitState, call_state::CallState,
        index_state::IndexState,
    },
    object::{ObjectRef, panic_obj::RuntimeSignal, state::StateRef},
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
            state: ExpressionState::Unary { value: None },
        }
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
                AwaitExpression::eval2(object.clone(), interpreter_state)?;
                *future = Some(object.clone());
                *state = AwaitState::Waiting;
                Ok(())
            }
            ExpressionState::Unary { value } => {
                *value = Some(object.clone());
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
            _ => Ok(()),
        }
    }
}

pub enum EvaluationResult {
    Done(ObjectRef),
    Pending,

    Push(Expression),
}
