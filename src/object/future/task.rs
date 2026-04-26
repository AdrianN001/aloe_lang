use crate::{ast::statement::Statement, object::{stack_environment::EnvRef, state::StateRef}};


#[derive(PartialEq, Eq, Clone, Default, Debug)]
pub struct Task{
    pub statement_index: usize,
    pub statements: Vec<Statement>,

    pub environ: EnvRef,
    pub state:   StateRef,
}
