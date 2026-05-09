use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::statement::Statement,
    frame::expr_frame::{EvaluationResult, ExpressionFrame},
    object::{
        Object, ObjectRef, new_objectref,
        stack_environment::{EnvRef, StackEnvironment},
    },
};

pub type BlockFrameRef = Rc<RefCell<BlockFrame>>;

#[derive(Debug)]
pub struct BlockFrame {
    pub statements: Vec<Statement>,
    pub index: usize,

    pub last_value: Option<ObjectRef>,

    pub environ: EnvRef,
    pub last_object: Option<ObjectRef>,
}

impl BlockFrame {
    pub fn new(statements: &[Statement], base_environment: EnvRef) -> Self {
        let environ = {
            let raw = StackEnvironment::new_enclosed(base_environment, String::from(""));
            Rc::new(RefCell::new(raw))
        };

        Self {
            statements: statements.to_vec(),
            index: 0,

            last_value: None,

            environ,
            last_object: None,
        }
    }

    pub fn eval_step(&mut self) -> EvaluationResult {
        if self.index >= self.statements.len() {
            return EvaluationResult::Done(match &self.last_object {
                Some(value) => value.clone(),
                None => new_objectref(Object::NULL_OBJECT),
            });
        }

        let stmt = self.statements[self.index].clone();

        self.eval_current_stmt(&stmt)
    }

    pub fn resume_with(&mut self, value: ObjectRef) -> Option<ObjectRef> {
        self.handle_statement_after_ready_value(value.clone())
    }

    fn eval_current_stmt(&mut self, statement: &Statement) -> EvaluationResult {
        match statement {
            Statement::Let(let_stmt) => {
                ExpressionFrame::build_frame_from_expr(&let_stmt.value, self.environ.clone())
            }
            Statement::Return(return_stmt) => {
                if let Some(return_value) = &return_stmt.value {
                    ExpressionFrame::build_frame_from_expr(&return_value, self.environ.clone())
                } else {
                    return EvaluationResult::Return(new_objectref(Object::NULL_OBJECT));
                }
            }
            Statement::Expression(expr_statement) => ExpressionFrame::build_frame_from_expr(
                &expr_statement.expression,
                self.environ.clone(),
            ),

            _ => todo!(),
        }
    }

    fn handle_statement_after_ready_value(&mut self, value: ObjectRef) -> Option<ObjectRef> {
        let current_statement = &self.statements[self.index];
        match current_statement {
            Statement::Let(let_stmt) => {
                self.environ.borrow_mut().set(&let_stmt.name.value, value);
                self.index += 1;
                None
            }
            Statement::Return(_) => {
                self.index += 1;
                Some(value)
            }
            _ => {
                self.last_object = Some(value.clone());
                self.index += 1;
                None
            }
        }
    }

    pub fn to_ref(self) -> BlockFrameRef {
        Rc::new(RefCell::new(self))
    }
}
