#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output};
use esp_hal::main;
use esp_println::println;


#[main]
fn main() -> ! {
    // TODO Blink the LED
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let mut led = Output::new(peripherals.GPIO35, Level::High);
    let delay = Delay::new();
    loop {
        led.toggle();
        println!("{}", led.is_set_high());
        delay.delay_millis(2000);
    }
}
