# TP 3 - First RTOS application

In this lab, we will create a simple RTOS application that will run on the ESP32s3 SoC. with using the Rust standard library to create tasks and make channel communication between them

## Multicore SoC and SMP

The ESP32s3 embeds 2 LX7 Tensica cores, respectively Core0 (PRO_CPU) and Core1 (APP_CPU).
In this session, we will leverage the Symmetric Multi-Processing (SMP) paradigm.
In SMP scenarios, each physical core is connected to a shared memory, and a single RTOS instance is running on the SoC. This allows the RTOS to schedule tasks on the different cores which are equally treated, i.e., no core has a special role.

## Task Creation

ESP-IDF is based on FreeRTOS, so it is possible to create tasks using the `xTaskCreatePinnedToCore` function and utilize the FreeRTOS API to manage tasks. However, ESP-IDF also provides support for the Rust standard library, allowing tasks to be created using Rust `thread::spawn` function. In this lab, **we chose to create tasks and maximize the use of the Rust standard library, as it is more generic and offers better safety guarantees.**

Nevertheless, the FreeRTOS API remains available and can be used in parallel with the Rust standard API.

### Example of task creation in C with FreeRTOS

```c
#include <stdio.h>
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

void task(void *pv_parameters) {
    while (1) {
        printf("Hello from the task!\n");
        vTaskDelay(1000 / portTICK_PERIOD_MS);
    }
}

void app_main(void) {
    xTaskCreatePinnedToCore(task, "task", 4096, NULL, 5, NULL, 1);
    while (1) {
        printf("Hello from the main loop!\n");
        vTaskDelay(1000 / portTICK_PERIOD_MS);
    }
}
```

### Example of task creation in RUST with FreeRTOS

```rust
use esp_idf_svc::{hal::delay::FreeRtos, sys::xTaskCreatePinnedToCore};

extern "C" fn task(_pv_parameters: *mut core::ffi::c_void) {
    loop {
        log::info!("Hello from the task!");
        FreeRtos::delay_ms(1000);
    }
}

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    unsafe {
        xTaskCreatePinnedToCore(Some(task),"task".as_ptr() as *const i8 as *mut i8,4096,core::ptr::null_mut(),5,core::ptr::null_mut(),1,);
    }

    loop {
        log::info!("Hello from the main loop!");
        FreeRtos::delay_ms(1000);
    }
}
```

### Example of task creation in Rust with the standard library

```rust
use std::thread;
use std::time::Duration;
use esp_idf_hal::task::thread::ThreadSpawnConfiguration;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    
    let mut thread_config = ThreadSpawnConfiguration::default();
    thread_config.name = Some("Interrupt_manger\0".as_bytes());
    thread_config.pin_to_core = Some(esp_idf_hal::cpu::Core::Core1);
    thread_config.set().unwrap();

    thread::spawn(|| {
        loop {
            log::info!("Hello from the task!");
            thread::sleep(Duration::from_secs(1));
        }
    });

    loop {
        log::info!("Hello from the main loop!");
        thread::sleep(Duration::from_secs(1));
    }
}
```

## Usefull links

+ [https://docs.esp-rs.org/esp-idf-hal/esp_idf_hal/task/thread/struct.ThreadSpawnConfiguration.html](https://docs.esp-rs.org/esp-idf-hal/esp_idf_hal/task/thread/struct.ThreadSpawnConfiguration.html)
+ [https://doc.rust-lang.org/std/thread/](https://doc.rust-lang.org/std/thread/)
+ [https://doc.rust-lang.org/std/sync/mpsc/index.html](https://doc.rust-lang.org/std/sync/mpsc/index.html)
+ [https://github.com/esp-rs/esp-idf-hal/blob/master/examples/button_interrupt.rs](https://github.com/esp-rs/esp-idf-hal/blob/master/examples/button_interrupt.rs)


## Part 1 : Sharing data between tasks

There exists multiple ways to share data between RTOS tasks, In this lab, we will use the Rust standard library channel to communicate between tasks. [https://doc.rust-lang.org/std/sync/mpsc/index.html](https://doc.rust-lang.org/std/sync/mpsc/index.html)

In this exercise, we will illustrate the usage of such channel to share data between to simple tasks.

```
┌−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−┐
╎                  CPU0             ╎
╎                                   ╎
╎ ┌───────────┐  channel   ┌──────┐ ╎
╎ │ generator │ ─────────► │ sink │ ╎
╎ └───────────┘            └──────┘ ╎
╎                                   ╎
└−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−┘
```

+ Complete the code of the `generator` and `sink` functions.
  + The `generator` function should send a message every second to the `sink` function.
  + The `sink` function should receive the message and print it.
+ In the main create a channel used to communicate between the two tasks.
+ Change the thread spawn configuration to pin the task to `Core0` [https://docs.esp-rs.org/esp-idf-hal/esp_idf_hal/task/thread/struct.ThreadSpawnConfiguration.html](https://docs.esp-rs.org/esp-idf-hal/esp_idf_hal/task/thread/struct.ThreadSpawnConfiguration.html)
+ Spawn the two tasks

## Part 2 : Multicore setup

Having two separate cores on the same SoC allows running two tasks at the same time. 
To illustrate this ability, we will schedule a new task on CPU1 of our ESP32s3 SoC.

```
┌−−−−−−−−−−−−−−−┐          ┌−−−−−−−−−−−−−−−┐
╎     CPU0      ╎          ╎     CPU1      ╎
╎ ┌───────────┐ ╎channel 1 ╎ ┌───────────┐ ╎
╎ │ generator │─────────────►│           │ ╎
╎ └───────────┘ ╎          ╎ │   relay   │ ╎
╎ ┌───────────┐ ╎channel 2 ╎ │           │ ╎
╎ │    sink   │◄─────────────│           │ ╎
╎ └───────────┘ ╎          ╎ └───────────┘ ╎
└−−−−−−−−−−−−−−−┘          └−−−−−−−−−−−−−−−┘
```

+ Create a new function `relay` that will receive a message from `generator`, prepend the message and send it to `sink`.
+ Change the first channel to communicate between `generator` and `relay`.
+ Create a new channel to communicate between `relay` and `sink`.
+ Change the thread spawn configuration to pin the task to `Core1`
+ Spawn the new relay task on `Core1`

## Part 3 : GPIO

In the previous parts of this sessions, we scheduled a periodic task and two aperiodic tasks triggered by presence of data in queues.

We will now show that tasks can also be triggered by external signals, such as interrupts generated by GPIO.

```
┌−−−−−−−−−−−−−−−┐          ┌−−−−−−−−−−−−−−−┐
╎     CPU0      ╎ New Task ╎     CPU1      ╎
╎ ┌───────────┐ ╎◄┄┄┄┄┄┄┄┄┄┄ ┌───────────┐ ╎ISR ┌────────┐
╎ │ generatorx│───────┐    ╎ │  spawner  │◄─────│ Button │
╎ └───────────┘ ╎     │    ╎ └───────────┘ ╎    └────────┘
╎ ┌───────────┐ ╎     │    ╎               ╎
╎ │ generator1│───────┤    ╎               ╎
╎ └───────────┘ ╎     │    ╎               ╎
╎ ┌───────────┐ ╎channel 1 ╎ ┌───────────┐ ╎
╎ │ generator0│───────┴─────►│           │ ╎
╎ └───────────┘ ╎          ╎ │   relay   │ ╎
╎ ┌───────────┐ ╎channel 2 ╎ │           │ ╎
╎ │    sink   │◄─────────────│           │ ╎
╎ └───────────┘ ╎          ╎ └───────────┘ ╎
└−−−−−−−−−−−−−−−┘          └−−−−−−−−−−−−−−−┘
```

+ Create a new function `spawner` that will spawn a new generator task every time the button is pressed.
  +  Create a Notification to notify the `spawner` task when the button is pressed.
  +  Register an interrupt handler on the button press event.
  +  Why do we need to use a notification instead of directly spawning the task in the interrupt handler?
+ Spawn the `spawner` task on `Core1`
+ We want to limit the number of generator tasks to 5. For this purpose, we will use static variable to count the number of generator tasks.
  + who can you guarantee no concurrent access to this variable?
