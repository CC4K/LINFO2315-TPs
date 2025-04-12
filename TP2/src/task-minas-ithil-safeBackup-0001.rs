use esp_hal::time::now;
use esp_println::println;

#[derive(Clone, Copy)]
pub enum TaskType{
    Periodic,
    Aperiodic
}

#[derive(Copy, Clone)]
pub struct Task {
    pub id: u32,
    pub task_type: TaskType,
    callback: fn(),
}

impl Task {
    pub fn new(id: u32, task_type: TaskType, callback: fn()) -> Self{
        Self{ id, task_type, callback }
    }

    pub fn run(self){
        (self.callback)()
    }
}
