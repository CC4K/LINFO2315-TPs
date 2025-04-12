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
        thread::spawn(move || tx.send("ok".to_owned()).unwrap());
        thread::sleep(Duration::from_secs(1));
    }
}

// Relay function
fn sink(rx: mpsc::Receiver<String>) {
    while let Ok(msg) = rx.recv() {
        log::info!("{msg}");
    }
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Pin to CPU 0
    let mut thread_config = ThreadSpawnConfiguration::default();
    thread_config.name = Some("Interrupt_manager\0".as_bytes());
    thread_config.pin_to_core = Some(esp_idf_hal::cpu::Core::Core0);
    thread_config.set().unwrap();

    // Configure channel between generator and sink 
    let (tx, rx) = channel();
    
    // Spawn the threads
    thread::spawn(|| {
        generator(tx);
    });
    thread::spawn(|| {
        sink(rx);
    });

}

