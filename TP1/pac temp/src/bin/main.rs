#![no_std]
#![no_main]

use esp32s3::Peripherals;
use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_println::println;

fn setup_gpio(p: &Peripherals) {
    // TODO Configure the GPIO as output
    p.GPIO.enable1().write(|w| unsafe { w.bits(1 << 3) });
    p.GPIO.func35_out_sel_cfg().write(|w|unsafe { w.bits(0x100) });
}

fn set_gpio(p: &Peripherals, val: bool) {
    // TODO Set the GPIO, HIGH if val is true, LOW if val is false
    if val {
        p.GPIO.out1_w1ts().write(|w| unsafe { w.bits(1 << 3) });
    }
    else {
        p.GPIO.out1_w1tc().write(|w| unsafe { w.bits(1 << 3) });
    }
}

fn setup_temp_sens(p: &Peripherals) {
    // SENS_SAR_TSENS_CTRL_REG => SENS_TSENS_POWER_UP_FORCE + SENS_TSENS_POWER_UP
    p.SENS.sar_tsens_ctrl().write(|w|
        w.sar_tsens_power_up().set_bit()
         .sar_tsens_power_up_force().set_bit() 
    );
    // SENS_SAR_POWER_XPD_SAR_REG => SENS_FORCE_XPD_SAR
    p.SENS.sar_power_xpd_sar().write(|w| unsafe { w.force_xpd_sar().bits(2) });
    // SENS_SAR_PERI_CLK_GATE_CONF_REG => SENS_TSENS_CLK_EN
    p.SENS.sar_peri_clk_gate_conf().modify(|_r, w| w.tsens_clk_en().set_bit() );
    // Wait
    Delay::new().delay_micros(3000);
}

fn read_temp_sens(p: &Peripherals) {
    // Read SENS_SAR_TSENS_CTRL_REG => SENS_TSENS_OUT
    p.SENS.sar_tsens_ctrl().modify(|_r,w|
        w.sar_tsens_dump_out().set_bit()
    );

    let delay = Delay::new();
    loop {
        let read = p.SENS.sar_tsens_ctrl().read();
        let ready = read.sar_tsens_ready().bit();
        if ready {
            println!("READY !");
            let value = read.sar_tsens_out().bits() as f64;
            let offset = 0.0;
            let value = 0.4386 * value - 27.88 * offset - 20.52;
            println!("Value: {}", value);
            p.SENS.sar_tsens_ctrl().modify(|_r, w| w.sar_tsens_dump_out().clear_bit());
            return;
        }
        else {
            println!("NOT READY !");
        }
        delay.delay_millis(1000);
    }
}

#[main]
fn main() -> ! {
    let _ = esp_hal::init(esp_hal::Config::default()); // Initialize the default configuration to disable the watchdog timer

    // TODO take Peripherals from PAC
    let peripherals =  Peripherals::take().unwrap();
    setup_gpio(&peripherals);
    setup_temp_sens(&peripherals);

    let mut is_on = false;
    let delay = Delay::new();
    loop {
        println!("Setting LED to {}", is_on);
        set_gpio(&peripherals, is_on);
        read_temp_sens(&peripherals);
        delay.delay_millis(2000);
        is_on = !is_on;
    }
}
