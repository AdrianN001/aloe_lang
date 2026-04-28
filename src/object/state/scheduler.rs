use std::{
    cell::RefCell,
    collections::VecDeque,
    time::{Duration, Instant},
};

use crate::object::{
    Object,
    future::{
        future_state::FutureState,
        task::{Task, TaskRef},
        task_kind::TaskKind,
    },
    panic_obj::RuntimeSignal,
};

thread_local! {
    pub static CURRENT_TASK: RefCell<Option<TaskRef>> = RefCell::new(None);
}

#[derive(PartialEq, Eq, Clone, Default, Debug)]
pub struct Scheduler {
    pub queue: VecDeque<TaskRef>,
    pub sleeping: Vec<(TaskRef, Instant)>,
    pub current_task: Option<TaskRef>,
}

impl Scheduler {
    pub fn run(&mut self) {
        loop {
            let now = Instant::now();

            // Wakeup prüfen
            let mut i = 0;
            while i < self.sleeping.len() {
                if self.sleeping[i].1 <= now {
                    let (task, _) = self.sleeping.remove(i);

                    {
                        let mut task_borrow = task.borrow_mut();
                        task_borrow.pending_future = None;
                        task_borrow.kind = None;
                        task_borrow.statement_index += 1;
                    }

                    self.queue.push_back(task);
                } else {
                    i += 1;
                }
            }

            if let Some(task) = self.queue.pop_front() {
                CURRENT_TASK.with(|slot| {
                    *slot.borrow_mut() = Some(task.clone());
                });

                let result = Task::run(task.clone());

                match result {
                    Ok(value) => {
                        let current_task_borrow = task.borrow();

                        if let Some(original_future) = &current_task_borrow.result_future {
                            let mut future_borrow = original_future.borrow_mut();

                            if let Object::Future(future_raw) = &mut *future_borrow {
                                future_raw.state = FutureState::Ready(value.clone());

                                future_raw.waiters.iter().for_each(|waiter_task| {
                                    {
                                        waiter_task.borrow_mut().pending_future =
                                            Some(original_future.clone());
                                    }

                                    self.queue.push_back(waiter_task.clone());
                                });

                                future_raw.waiters.clear();
                            }
                        }
                        println!(
                            "task ist fertig. Wert: {}, blieb:{}",
                            value.borrow().inspect(),
                            self.queue.len()
                        );
                    }

                    Err(RuntimeSignal::Yield(t)) => {
                        let new_task_borrow = t.borrow();
                        if let Some(kind) = &new_task_borrow.kind {
                            match kind {
                                TaskKind::Sleep(wait_until) => {
                                    self.sleeping.push((task.clone(), *wait_until));
                                }
                                TaskKind::Value(new_awaited_taskref) => {
                                    self.queue.push_back(new_awaited_taskref.clone());
                                }
                            }
                        } else {
                            self.queue.push_back(task.clone());
                        }
                    }

                    Err(RuntimeSignal::Panic(p)) => {
                        eprintln!("Runtime error: {:?}", p.inspect_message());
                    }
                }

                CURRENT_TASK.with(|slot| {
                    *slot.borrow_mut() = None;
                });
            } else if !self.sleeping.is_empty() {
                std::thread::sleep(Duration::from_millis(1));
            } else {
                break;
            }
        }
    }
}

pub fn set_current_task(task: TaskRef) {
    CURRENT_TASK.with(|slot| {
        *slot.borrow_mut() = Some(task);
    });
}

pub fn take_current_task() -> Option<TaskRef> {
    CURRENT_TASK.with(|slot| slot.borrow().clone())
}

pub fn clear_current_task() {
    CURRENT_TASK.with(|slot| {
        *slot.borrow_mut() = None;
    });
}
