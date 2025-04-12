#![no_std]
#![no_main]

use esp32s3::Peripherals;
use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_println::println;


fn setup_gpio(p: &Peripherals) {
    // TODO Configure the GPIO as output
    // GPIO_ENABLE1_REG
    p.GPIO.enable1().write(|w| unsafe { w.bits(1 << 3) });

    // GPIO_FUNC35_OUT_SEL_CFG_REG
    p.GPIO.func35_out_sel_cfg().write(|w| unsafe { w.bits(256) });
}

fn set_gpio(p: &Peripherals, val: bool) {
    // TODO Set the GPIO, HIGH if val is true, LOW if val is false
    // GPIO_OUT1_REG
    if val { p.GPIO.out1().write(|w| unsafe { w.bits(1 << 3) }); } // 32 => 35
    else { p.GPIO.out1().write(|w| unsafe { w.bits(0 << 3) }); }
}

#[main]
fn main() -> ! {
    let _ = esp_hal::init(esp_hal::Config::default()); // Initialize the default configuration to disable the watchdog timer

    // TODO take Peripherals from PAC
    let peripherals =  Peripherals::take().unwrap();

    setup_gpio(&peripherals);

    let mut is_on = false;
    let delay = Delay::new();
    loop {
        set_gpio(&peripherals, is_on);
        delay.delay_millis(2000);
        is_on = !is_on;
        println!("{}", is_on);
    }
}
