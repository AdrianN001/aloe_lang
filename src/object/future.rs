use crate::object::future::{future_state::FutureState, task::TaskRef};

pub mod future_kind;
pub mod future_state;
pub mod sleep;
pub mod task;
pub mod task_kind;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FutureObj {
    pub state: FutureState,

    pub waiters: Vec<TaskRef>,
}

impl FutureObj {
    pub fn get_type(&self) -> String {
        String::from("<type 'future'>")
    }

    pub fn inspect(&self) -> String {
        self.get_type()
    }
}
