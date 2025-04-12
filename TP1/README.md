# TP1: Three Levels of Abstraction to Blink an LED

In this lab, we will learn how to blink an LED using three different levels of abstraction. We will start with the lowest level, which involves directly manipulating the microcontroller's registers. Then, we will use a Peripheral Access Crate (PAC). Finally, we will use the Hardware Abstraction Layer (HAL) library.

## Useful Documentation

- [ESP32-S3 Technical Reference Manual](https://www.espressif.com/sites/default/files/documentation/esp32-s3_technical_reference_manual_en.pdf)
  - Don't panic, you don't need to read everything, but it's useful to know where to find the necessary information.
  - For this lab, you will need to refer to **Chapter 6** IO MUX and GPIO Matrix (GPIO, IO MUX), particularly **Section 6.5** Peripheral Output via GPIO Matrix. You will also need to use **Chapter 4** for register addresses.
- Pin map of your ESP32-S3 board
  - ![pin map](https://resource.heltec.cn/download/WiFi_LoRa_32_V3/HTIT-WB32LA(F)_V3.png)
- PACs documentation
  - [svd2rust](https://docs.rs/svd2rust/latest/svd2rust/)
  - The **Peripheral API** and **read/modify/write API** sections are the most useful.
- Espressif HAL documentation
  - [esp-hal](https://docs.esp-rs.org/esp-hal/esp-hal/0.23.1/esp32s3/esp_hal/)

## Level 1: Direct Register Manipulation

In this first level, we will directly manipulate the microcontroller's registers to blink an LED. To do this, we will use the Rust programming language.

### Register Manipulation in Rust

Example of register manipulation in Rust:

```rust
const REGISTRE: *mut u32 = (base_addr + offset) as *mut u32;
unsafe {
    REGISTRE.write_volatile(valeur);
}
```

### Development Environment

We have already set up a development environment for you and written a base code to help you start. To begin, you need to:

- Go to the `unsafe` directory and open VSCode. You can open VSCode by typing `code .` in the terminal.

### Tasks

Here, you need to blink an LED using direct register manipulation. To do this, you must:

- Identify the GPIO connected to the white LED on the ESP32-S3 board
- Identify the registers needed to configure the GPIO in output mode
- Identify the registers needed to turn the GPIO on and off
- Turn the LED on and off every 2 seconds

Different `TODO` comments in the code will guide you on where to add code.

## Level 2: Peripheral Access Crates (PACs)

In this second level, we will use a Peripheral Access Crate (PAC) to blink an LED.
The PAC is a library generated automatically from the microcontroller's SVD (System View Description) file. This library provides types and functions to access the microcontroller's registers in a more or less secure way.

### Development Environment

We have already set up a development environment for you and written a base code to help you start. To begin, you need to:

- Go to the `pac` directory and open VSCode. You can open VSCode by typing `code .` in the terminal.

### Tasks

Here, you need to blink an LED using a PAC. To do this, you must:

- Initialize the PAC to retrieve the microcontroller's registers
- Configure the GPIO connected to the white LED on the ESP32-S3 board in output mode
- Turn the LED on and off every 2 seconds

Different `TODO` comments in the code will guide you on where to add code.

## Level 3: HAL (Hardware Abstraction Layer)

In this third level, we will use the Hardware Abstraction Layer (HAL) library to blink an LED.

### Development Environment

We have already set up a development environment for you and written a base code to help you start. To begin, you need to:

- Go to the `hal` directory and open VSCode. You can open VSCode by typing `code .` in the terminal.

### Tasks

Here, you need to blink an LED using the HAL library. To do this, you must:

- Initialize the HAL
- Create a GPIO Output object for the GPIO connected to the white LED on the ESP32-S3 board
- Turn the LED on and off every 2 seconds

Different `TODO` comments in the code will guide you on where to add code.

## Conclusion

In this lab, we have learned how to blink an LED using three different levels of abstraction. We started with the lowest level, which involves directly manipulating the microcontroller's registers. Then, we used a Peripheral Access Crate (PAC). Finally, we used the Hardware Abstraction Layer (HAL) library. This allowed us to see the advantages and disadvantages

## Extra

If you have completed the lab, you can try to retrieve the temperature of the microcontroller using the registers first and then the PACs.

To do this, you will need to read Section **39.4 Temperature Sensor** of Chapter 39 of the ESP32-S3 technical documentation.