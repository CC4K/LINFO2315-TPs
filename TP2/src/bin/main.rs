#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use core::cell::RefCell;
use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Event, Input, Io, Level, Output, Pull};
use esp_hal::handler;
use esp_hal::interrupt::InterruptConfigurable;
use esp_hal::main;
use esp_hal::xtensa_lx;
use log::info;
use simple_sched::sched::Sched;
use simple_sched::task::Task;
use esp_hal::trapframe::TrapFrame;
use esp_hal::xtensa_lx::timer::delay;
use esp_hal::xtensa_lx_rt::exception::Context;
use esp_println::println;
//static BUTTON // TODO: 2.1: Declare a static variable for the button
//static LED // TODO: 2.1: Declare a static variable for the LED


//2.1 Write a handler function that toggles the LED when the button is pressed


#[main]
fn main() -> ! {
    fn task1() {
        println!("Task 1 running...");
        delay(1000)
    }
    fn task2() {
        println!("Task 2 running...");
        delay(2000)
    }
    fn task3() {
        println!("Task 3 running...");
        delay(3000)
    }

    let peripherals = esp_hal::init(esp_hal::Config::default()); // Initialize the default configuration to disable the watchdog timer

    esp_println::logger::init_logger_from_env();
    

    // let mut led = Output::new(peripherals.GPIO35, Level::High);
    // let delay = Delay::new();
    // loop {
    //     led.toggle();
    //     println!("{}", led.is_set_high());
    //     delay.delay_millis(2000);
    // }

    // TODO: 2.1: Initialize the IO Multiplexer and set the interrupt handler

    
    // TODO: 2.1: Initialize the LED and Button peripherals use the button and the led embedded in the development board

    // TODO: 2.1: Set the static variables BUTTON and LED

    // TODO: 1.2: Create a new Scheduler and add three tasks that delay for 1, 2, and 3 seconds respectively
    let mut scheduler = Sched::new();
    scheduler.add_task(Task::new(1, task1));
    scheduler.add_task(Task::new(2, task2));
    scheduler.add_task(Task::new(3, task3));
    loop {
        // TODO: 1.2: Run the Scheduler and print the time it took to run the tasks
        let start = esp_hal::time::now();
        scheduler.run();
        let end = esp_hal::time::now();
        let elapsed = end - start;
        println!("Scheduler cycle executed in {}", elapsed);
    }
}
