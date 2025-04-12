use crate::task::{Task, TaskType};

const MAX_TASKS: usize = 32;

#[derive(Debug)]
pub enum SchedAperiodicErr{
    NoTask,
    Full
}

pub struct SchedAperiodic {
    tasks: [Option<Task>; MAX_TASKS],
    current_task: usize,
    last_task: usize,
}

impl Default for SchedAperiodic {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedAperiodic{
    pub fn new() -> Self {
        Self{
            tasks: [None; MAX_TASKS],
            current_task: 0,
            last_task: 0
        }
    }

    pub fn add_task(&mut self, task: Task) -> Result<(), SchedAperiodicErr> {
        let start = self.last_task % MAX_TASKS;
        let mut curr = start;
        while let Some(_) = self.tasks[curr]{
            curr = (curr + 1) % self.last_task;
            if curr == start{
                return Err(SchedAperiodicErr::Full);
            }
        }

        self.last_task = self.last_task.max(curr + 1);
        self.tasks[curr] = Some(task);
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), SchedAperiodicErr> {
        let start = self.current_task;
        let mut curr = start;
        while let None = self.tasks[curr]{
            curr = (curr + 1) % self.last_task;
            if curr == start {
                return Err(SchedAperiodicErr::NoTask)
            }
        }
        let task = self.tasks[curr].unwrap();
        task.run();
        if matches!(task.task_type, TaskType::Aperiodic) {
            self.tasks[self.current_task] = None;
        }
        self.current_task = (self.current_task + 1) % self.last_task;
        Ok(())
    }
}
