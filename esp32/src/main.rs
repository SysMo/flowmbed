use std::thread;

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::gpio::PinDriver;
use anyhow::Result;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use services::mqtt_handler::SimpleHandler;
mod services;
use std::time::Duration;
use log::*;
use esp_idf_svc::nvs::{EspNvsPartition, NvsDefault};

use crate::services::mqtt_handler::MQTTHandler;


fn main() -> Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals: Peripherals = Peripherals::take().unwrap();
       
    let mut indicator_led = PinDriver::output(peripherals.pins.gpio2)?;
    indicator_led.set_high()?;

    
    let sysloop = EspSystemEventLoop::take()?;

    let nvs = EspNvsPartition::<NvsDefault>::take()?;


    let wifi = services::wifi::start(peripherals.modem, sysloop.clone())?;

    let mqtt_handler: SimpleHandler = services::mqtt_handler::SimpleHandler::new();
    // let mqtt_handler = services::mqtt_handler::SimpleHandler::new();
    let mqtt = services::mqtt::start(mqtt_handler.inbox());

    info!("Initialization completed");

    loop {
        thread::sleep(Duration::from_millis(100));
        match mqtt_handler.check_inbox() {
            Some(msg) => {
                info!("Message received: {}", msg);
                match msg.as_str() {
                    "on" => indicator_led.set_high(),
                    "off" => indicator_led.set_low(),
                    "toggle" => indicator_led.toggle(),
                    _ => Ok(())
                };
            }
            None => ()
        }
        
    }

    Ok(())
}
