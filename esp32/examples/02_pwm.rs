
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::ledc::{LedcDriver, LedcTimerDriver, config};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::timer;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let mut channel = LedcDriver::new(
        peripherals.ledc.channel0,
        LedcTimerDriver::new(
            peripherals.ledc.timer0,
            &config::TimerConfig::new().frequency(25.kHz().into()),
        )?,
        peripherals.pins.gpio4,
    )?;
    
    let max_duty = channel.get_max_duty();
    println!("{}", max_duty);
    channel.set_duty( max_duty / 2)?;
    
    FreeRtos::delay_ms(2000);

    loop {
        FreeRtos::delay_ms(1000);
    }
    Ok(()) 
}