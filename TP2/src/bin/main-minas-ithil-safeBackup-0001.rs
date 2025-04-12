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
//use simple_sched::task::Task;
use esp_hal::trapframe::TrapFrame;
use esp_hal::xtensa_lx::timer::delay;
use esp_hal::xtensa_lx_rt::exception::Context;
use esp_println::println;
use simple_sched::task::{Task, TaskType};

//2.1 Write a handler function that toggles the LED when the button is pressed
static BUTTON: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None)); // TODO: 2.1: Declare a static variable for the button
static LED: Mutex<RefCell<Option<Output>>> = Mutex::new(RefCell::new(None)); // TODO: 2.1: Declare a static variable for the LED

fn button_is_interrupt() -> bool{
    critical_section::with(|cs| {
        BUTTON.borrow_ref_mut(cs).as_mut().unwrap().is_interrupt_set()
    })
}

#[handler]
fn button_pressed() {
    if button_is_interrupt() {
        println!("Interrupt");
        critical_section::with(|cs| {
            LED.borrow_ref_mut(cs).as_mut().unwrap().toggle();
        });
    }

    /* 
    for _ in 0..1_000_000_000 {
        unsafe {
            asm!("nop");
        }  
    }
    */

    critical_section::with(|cs| {
        BUTTON.borrow_ref_mut(cs).as_mut().unwrap().clear_interrupt()
    });
}

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default()); // Initialize the default configuration to disable the watchdog timer
    esp_println::logger::init_logger_from_env();

    // TODO: 2.1: Initialize the IO Multiplexer and set the interrupt handler
    let mut io = Io::new(peripherals.IO_MUX);
    io.set_interrupt_handler(button_pressed);

    // TODO: 2.1: Initialize the LED and Button peripherals use the button and the led embedded in the development board
    let led = Output::new(peripherals.GPIO35, Level::Low);
    let mut button = Input::new(peripherals.GPIO0, Pull::Up);
    button.listen(Event::FallingEdge);

    // TODO: 2.1: Set the static variables BUTTON and LED
    critical_section::with(|cs| {
        LED.borrow_ref_mut(cs).replace(led);
        BUTTON.borrow_ref_mut(cs).replace(button);
    });

    // TODO: 1.2: Create a new Scheduler and add three tasks that delay for 1, 2, and 3 seconds respectively
    let mut scheduler = Sched::new();
    scheduler.add_task(Task::new(1, TaskType::Periodic, || {
        Delay::new().delay_millis(1000);
        println!("Task 1")
    })).unwrap();
    scheduler.add_task(Task::new(2, TaskType::Periodic, || {
        Delay::new().delay_millis(2000);
        println!("Task 2")
    })).unwrap();
    scheduler.add_task(Task::new(3, TaskType::Aperiodic, || {
        Delay::new().delay_millis(3000);
        println!("Task 3")
    })).unwrap();

    loop {
        // TODO: 1.2: Run the Scheduler and print the time it took to run the tasks
        let start = esp_hal::time::now();
        scheduler.run();
        let end = esp_hal::time::now();
        let elapsed = end - start;
        println!("Scheduler cycle executed in {} second(s)", elapsed.to_secs());
    }
}
