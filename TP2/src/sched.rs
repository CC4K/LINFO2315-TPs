//use alloc::string::String;
use esp_hal::twai::Id;
use esp_hal::xtensa_lx::timer::delay;
use esp_println::println;
use log::info;
use esp_hal::main;

use crate::task::Task;

const MAX_TASKS: usize = 32;

pub struct Sched {
    tasks: Option<[Task; MAX_TASKS]>,
    current_task: usize,
    last_task: usize,
}

impl Sched {
    pub fn new() -> Self {
        Sched {
            tasks: None,
            current_task: 0,
            last_task: 0,
        }
    }

    pub fn add_task(&mut self, task: Task) {
        if self.tasks.is_none() {
            self.tasks = Some([task; MAX_TASKS]);
            self.last_task = 1;
        }
        else if let Some(ref mut tasks) = self.tasks {
            if self.last_task < MAX_TASKS {
                tasks[self.last_task] = task;
                self.last_task += 1;
            }
            else {
                println!("Scheduler is full. Cannot add more tasks.");
            }
        }
    }

    pub fn run(&mut self) {
        if let Some(ref tasks) = self.tasks {
            if self.current_task < self.last_task {
                let task = &tasks[self.current_task];
                println!("Running Task {}", task.id);
                task.run();
                self.current_task = (self.current_task + 1) % self.last_task;
            }
            else {
                println!("No tasks to run.");
            }
        }
        else {
            println!("No tasks in the scheduler.");
        }
    }
}
