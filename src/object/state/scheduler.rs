use std::{
    cell::RefCell,
    collections::VecDeque,
    time::{Duration, Instant},
};

use crate::object::{
    future::{
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

                    task.borrow_mut().kind = None;

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
                                _ => {
                                    self.queue.push_back(task.clone());
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
                println!("Aasd");
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
