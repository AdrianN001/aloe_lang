use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    sync::mpsc::{Receiver, Sender},
    time::{Duration, Instant},
};

pub mod message_output;

use crate::object::{
    Object, ObjectRef,
    future::{
        future_state::FutureState,
        task::{Task, TaskRef},
        task_kind::TaskKind,
    },
    panic_obj::RuntimeSignal,
    state::scheduler::message_output::MessageOutput,
};

thread_local! {
    pub static CURRENT_TASK: RefCell<Option<TaskRef>> = RefCell::new(None);
    pub static GLOBAL_SCHEDULER: RefCell<Scheduler> = RefCell::new(Scheduler::default());

    pub static TOKIO_RUNTIME: RefCell<tokio::runtime::Runtime> = RefCell::new(tokio::runtime::Runtime::new().unwrap());
    pub static SCHEDULER_CHANNEL: RefCell<(Sender<(u64, MessageOutput)>, Receiver<(u64, MessageOutput)>)> = RefCell::new(std::sync::mpsc::channel());
    pub static IO_FUTURES: RefCell<HashMap<u64, ObjectRef>> = RefCell::new(HashMap::<u64, ObjectRef>::new());

    pub static TEMP_SCHEDULER_QUEUE: RefCell<VecDeque<TaskRef>> = RefCell::new(VecDeque::new());
}

#[derive(Debug)]
pub struct Scheduler {
    pub main_queue: VecDeque<TaskRef>,
    pub sleeping: Vec<(TaskRef, Instant)>,
}

impl Scheduler {
    pub fn run(&mut self) -> Result<(), RuntimeSignal> {
        loop {
            let now = Instant::now();

            self.check_if_new_tasks_added();

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

                    self.main_queue.push_back(task);
                } else {
                    i += 1;
                }
            }
            if let Some((future_id, message)) = try_get_message_from_scheduler() {
                if let Some(future_ref) = remove_io_future(&future_id) {
                    let mut future_borrow = future_ref.borrow_mut();

                    if let Object::Future(future_obj) = &mut *future_borrow {
                        future_obj.state = FutureState::Ready(message.to_objectref()?);

                        future_obj.waiters.iter().for_each(|waiter_task| {
                            {
                                waiter_task.borrow_mut().pending_future = Some(future_ref.clone());
                            }

                            self.main_queue.push_back(waiter_task.clone());
                        });

                        future_obj.waiters.clear();
                    }
                }
            }

            if let Some(task) = self.main_queue.pop_front() {
                CURRENT_TASK.with(|slot| {
                    *slot.borrow_mut() = Some(task.clone());
                });

                {
                    let task_borrow = task.borrow();
                    let task_name = task_borrow.name.clone();
                    let state = task_borrow.state.clone();
                    state.borrow_mut().push_to_stack(task_name);
                }
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

                                    self.main_queue.push_back(waiter_task.clone());
                                });

                                future_raw.waiters.clear();
                            }
                        }
                    }

                    Err(RuntimeSignal::Yield(t)) => {
                        let new_task_borrow = t.borrow();
                        if let Some(kind) = &new_task_borrow.kind {
                            match kind {
                                TaskKind::Sleep(wait_until) => {
                                    self.sleeping.push((task.clone(), *wait_until));
                                }
                                TaskKind::ValueJoin(new_awaited_taskref) => {
                                    self.main_queue.push_back(new_awaited_taskref.clone());
                                }
                                TaskKind::FileIOJoin => {}
                            }
                        } else {
                            self.main_queue.push_back(task.clone());
                        }
                    }

                    Err(RuntimeSignal::Panic(p)) => {
                        return Err(RuntimeSignal::Panic(p));
                    }
                }

                {
                    let task_borrow = task.borrow();
                    let state = task_borrow.state.clone();
                    state.borrow_mut().pop_from_stack();
                }

                CURRENT_TASK.with(|slot| {
                    *slot.borrow_mut() = None;
                });
                // I/O-Tasks
            } else if !self.sleeping.is_empty() || !io_future_empty() {
                std::thread::sleep(Duration::from_millis(1));
            } else {
                break Ok(());
            }
        }
    }

    fn check_if_new_tasks_added(&mut self) {
        let new_tasks = get_tasks_from_temp_scheduler_queue();
        if let Some(tasks) = new_tasks {
            for task in tasks {
                self.main_queue.push_back(task);
            }
        }
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Scheduler {
            main_queue: VecDeque::new(),
            sleeping: Vec::new(),
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

pub fn add_task_to_scheduler(task: TaskRef) {
    GLOBAL_SCHEDULER.with(|scheduler| {
        scheduler.borrow_mut().main_queue.push_back(task);
    });
}

pub fn try_get_message_from_scheduler() -> Option<(u64, MessageOutput)> {
    SCHEDULER_CHANNEL.with(|slot| slot.borrow().1.try_recv().ok())
}

pub fn add_io_future(future_id: u64, future_ref: ObjectRef) {
    IO_FUTURES.with(|slot| {
        slot.borrow_mut().insert(future_id, future_ref);
    });
}

pub fn remove_io_future(future_id: &u64) -> Option<ObjectRef> {
    IO_FUTURES.with(|slot| slot.borrow_mut().remove(future_id))
}

pub fn io_future_empty() -> bool {
    IO_FUTURES.with(|slot| slot.borrow().is_empty())
}

pub fn send_task_to_scheduler(task: TaskRef) {
    TEMP_SCHEDULER_QUEUE.with(|slot| {
        slot.borrow_mut().push_back(task);
    });
}

pub fn get_tasks_from_temp_scheduler_queue() -> Option<Vec<TaskRef>> {
    TEMP_SCHEDULER_QUEUE.with(|slot| {
        let mut queue = slot.borrow_mut();
        if queue.is_empty() {
            return None;
        }
        let tasks: Vec<TaskRef> = queue.drain(..).collect();
        Some(tasks)
    })
}
