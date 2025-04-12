use esp_hal::time::now;
use esp_println::println;

#[derive(Copy, Clone)]
pub struct Task {
    pub id: u32,
    callback: fn(),
}

impl Task {
    pub fn new(id: u32, callback: fn()) -> Self {
        Task { id, callback }
    }

    pub fn run(&self) {
        let start = now();
        (self.callback)();
        let end = now();
        let elapsed = end - start;
        println!("Task {} executed in {}", self.id, elapsed);
    }
}
