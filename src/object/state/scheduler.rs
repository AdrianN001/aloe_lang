use std::collections::VecDeque;

use crate::object::future::task::Task;


#[derive(PartialEq, Eq, Clone, Default, Debug)]
pub struct Scheduler{
    queue: VecDeque<Task> 
}
