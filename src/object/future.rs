use crate::object::future::{future_state::FutureState, task::TaskRef};

pub mod future_kind;
pub mod future_state;
pub mod sleep;
pub mod task;
pub mod task_kind;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FutureObj {
    pub state: FutureState,

    id: u64, // unique identifier for the future, used for tracking and debugging
    pub waiters: Vec<TaskRef>,
}

impl FutureObj {
    fn generate_id() -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static FUTURE_ID_COUNTER: AtomicU64 = AtomicU64::new(1);
        FUTURE_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn new(state: FutureState) -> Self {
        FutureObj {
            state,
            id: Self::generate_id(),
            waiters: Vec::new(),
        }
    }
}

impl FutureObj {
    pub fn get_type(&self) -> String {
        String::from("<type 'future'>")
    }

    pub fn inspect(&self) -> String {
        format!("Future(id={})", self.id)
    }
}
