use std::time::{Duration, Instant};

use crate::object::{
    future::{FutureObj, future_kind::FutureKind, future_state::FutureState},
    stack_environment::EnvRef,
    state::StateRef,
};

impl FutureObj {
    pub fn new_sleep(sleep_ms: i64, _environ: EnvRef, _state: StateRef) -> Self {
        let duration = Duration::from_millis(sleep_ms as u64);
        let now = Instant::now();

        Self::new(FutureState::Pending(FutureKind::Sleep(now + duration)))
    }
}
