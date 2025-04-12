use std::num::NonZero;
use std::thread;
use std::sync::{mpsc, Mutex};
use std::time::Duration;
use esp_idf_hal::gpio::{Gpio0, Input, InterruptType, PinDriver, Pull};
use esp_idf_hal::task::notification::Notification;
use esp_idf_hal::task::thread::ThreadSpawnConfiguration;
use std::sync::mpsc::channel;

const MAX_GENERATORS: u8 = 5;
static NUMBER_GENERATORS: Mutex<u8> = Mutex::new(0);

// Generator function
fn generator(tx: mpsc::Sender<String>, msg: String) {
    // Sends a message to the other thread every second
    for _ in 0..MAX_GENERATORS {
        // let tx = tx.clone();
        // thread::spawn(move || tx.send("Hello".to_owned()).unwrap());
        if let Err(_) = tx.send(msg.clone()) {
            log::error!("Failed to send message");
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }
    *NUMBER_GENERATORS.lock().unwrap() -= 1;
}

// Relay function
fn relay(rx: mpsc::Receiver<String>, tx: mpsc::Sender<String>) {
    while let Ok(msg) = rx.recv() {
        log::info!("[RELAY] Received msg: '{msg}'. Now transmitting msg...");
        if let Err(_) = tx.send(msg) {
            log::error!("Failed to send message");
            break;
        }
    }
}

// Relay function
fn sink(rx: mpsc::Receiver<String>) {
    while let Ok(msg) = rx.recv() {
        log::info!("[SINK] Received msg: {msg}");
    }
}

// Spawner function
fn spawner(tx: mpsc::Sender<String>) {
    let gpio = unsafe{ Gpio0::new() };
    let mut input = PinDriver::input(gpio).unwrap();
    input.set_pull(Pull::Down).unwrap();
    input.set_interrupt_type(InterruptType::PosEdge).unwrap();
    
    let mut count = 0;
    loop {
        let notification = enable_interrupt(&mut input);
        notification.wait_any();

        let mut number_generators = NUMBER_GENERATORS.lock().unwrap();

        if *number_generators < MAX_GENERATORS {
            ThreadSpawnConfiguration {
                name: Some("Generator\0".as_bytes()),
                pin_to_core: Some(esp_idf_hal::cpu::Core::Core0),
                ..Default::default()
            }.set().unwrap();

            let tx_clone = tx.clone();
            thread::spawn(move || {
                generator(tx_clone, format!("{count}"));
            });

            count += 1;
            *number_generators += 1;
        }
    }
}

fn enable_interrupt(input: &mut PinDriver<'_, Gpio0, Input>) -> Notification {
    let notification = Notification::new();
    let waker = notification.notifier();

    unsafe {
        input.subscribe_nonstatic(move || {
            waker.notify(NonZero::new(1).unwrap());
        }).unwrap();
    }

    input.enable_interrupt().unwrap();
    notification
}


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Create channels to communicate between the two threads
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();
    
    // Configure and spawn generator thread
    ThreadSpawnConfiguration {
        name: Some("Spawner\0".as_bytes()),
        pin_to_core: Some(esp_idf_hal::cpu::Core::Core1),
        ..Default::default()
    }.set().unwrap();
    thread::spawn(|| {
        // log::info!("Spawner ready...");
        spawner(tx1);
    });

    // Configure and spawn relay thread
    ThreadSpawnConfiguration {
        name: Some("Relay\0".as_bytes()),
        pin_to_core: Some(esp_idf_hal::cpu::Core::Core1),
        ..Default::default()
    }.set().unwrap();
    thread::spawn(|| {
        log::info!("Relay ready...");
        relay(rx1, tx2);
    });

    // Configure and spawn sink thread
    ThreadSpawnConfiguration {
        name: Some("Receiver\0".as_bytes()),
        pin_to_core: Some(esp_idf_hal::cpu::Core::Core0),
        ..Default::default()
    }.set().unwrap();
    thread::spawn(|| {
        log::info!("Receiver ready...");
        sink(rx2);
    });

    log::info!("Main thread finished");

}
