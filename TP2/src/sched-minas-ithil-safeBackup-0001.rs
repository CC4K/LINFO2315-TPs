//use alloc::string::String;
use esp_hal::twai::Id;
use esp_hal::xtensa_lx::timer::delay;
use esp_println::println;
use log::info;
use esp_hal::main;

use crate::task::Task;

const MAX_TASKS: usize = 32;

#[derive(Debug)]
pub enum SchedErr {
    NoTask,
    Full
}

pub struct Sched {
    tasks: [Option<Task>; MAX_TASKS],
    current_task: usize,
    last_task: usize,
}

impl Default for Sched {
    fn default() -> Self {
        Self::new()
    }
}

impl Sched{
    pub fn new() -> Self {
        Sched{
            tasks: [None; MAX_TASKS],
            current_task: 0,
            last_task: 0
        }
    }

    pub fn add_task(&mut self, task: Task) -> Result<(), SchedErr> {
        if self.last_task >= MAX_TASKS{
            return Err(SchedErr::Full);
        }
        self.tasks[self.last_task] = Some(task);
        self.last_task += 1;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), SchedErr> {
        match self.tasks[self.current_task]{
            Some(task) => {
                task.run();
                self.current_task = (self.current_task + 1) % self.last_task;
                Ok(())
            },
            None => Err(SchedErr::NoTask),
        }
    }
}
