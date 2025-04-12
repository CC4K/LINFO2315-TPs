use std::num::NonZero;
use std::thread;
use std::sync::{mpsc, Mutex};
use std::time::Duration;
use esp_idf_hal::gpio::{Gpio0, Input, InterruptType, PinDriver, Pull};
use esp_idf_hal::task::notification::Notification;
use esp_idf_hal::task::thread::ThreadSpawnConfiguration;
use std::sync::mpsc::channel;

const MAX_GENERATORS: u8 = 5;

// Generator function
fn generator(tx: mpsc::Sender<String>) {
    // Sends a message to the other thread every second
    for _ in 0..MAX_GENERATORS {
        let tx = tx.clone();
        thread::spawn(move || tx.send("Hello".to_owned()).unwrap());
        thread::sleep(Duration::from_secs(1));
    }
}

// Relay function
fn relay(rx: mpsc::Receiver<String>, tx: mpsc::Sender<String>) {
    while let Ok(msg) = rx.recv() {
        log::info!("[RELAY] Received msg: '{msg}'. Now transmitting msg...");
        tx.send(msg).unwrap();
    }
}

// Relay function
fn sink(rx: mpsc::Receiver<String>) {
    while let Ok(msg) = rx.recv() {
        log::info!("[SINK] Received msg: {msg}");
    }
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
        name: Some("Generator\0".as_bytes()),
        pin_to_core: Some(esp_idf_hal::cpu::Core::Core0),
        ..Default::default()
    }.set().unwrap();
    thread::spawn(|| {
        log::info!("Generator ready...");
        generator(tx1);
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
