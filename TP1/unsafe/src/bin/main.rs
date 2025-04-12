#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    main,
};
// use esp_hal::delay::Delay;
// use esp_hal::entry;
use esp_println::println;


fn setup_gpio() {
    // TODO Configure the GPIO as output
    // GPIO_ENABLE1_REG
    const GPIO_ENABLE1_REG: *mut u32 = (0x6000_4000 + 0x002C) as *mut u32;
    unsafe { GPIO_ENABLE1_REG.write_volatile(1 << 3); } // 32 => 35
    // GPIO_FUNC35_OUT_SEL_CFG_REG
    const GPIO_FUNC35_OUT_SEL_CFG_REG: *mut u32 = (0x6000_4000 + 0x0554 + 0x8C) as *mut u32; // 4*23
    unsafe { GPIO_FUNC35_OUT_SEL_CFG_REG.write_volatile(256); }
}

fn set_gpio(val: bool) {
    // TODO Set the GPIO, HIGH if val is true, LOW if val is false
    // GPIO_OUT1_REG
    const GPIO_OUT1_REG: *mut u32 = (0x6000_4000 + 0x0010) as *mut u32;
    unsafe {
        if val { GPIO_OUT1_REG.write_volatile(1 << 3); } // 32 => 35
        else { GPIO_OUT1_REG.write_volatile(0 << 3); }
    }
}

#[main]
fn main() -> ! {
    let _ = esp_hal::init(esp_hal::Config::default()); // Initialize the default configuration to disable the watchdog timer

    setup_gpio(); // Set the gpio as output

    let mut is_on = true;
    let delay = Delay::new();
    loop {
        set_gpio(is_on); // Set the value of the GPIO
        delay.delay_millis(2000);
        is_on = !is_on;
        println!("{}", is_on);
    }

    // sudo usermod -a -G dialout $USER
    // newgrp dialout
    //
    // cargo run --release
}
