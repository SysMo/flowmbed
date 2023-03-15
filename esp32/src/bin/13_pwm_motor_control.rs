use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio;
use esp_idf_hal::adc;
use esp_idf_hal::units;
use flowmbed_peripherals::actuators::traits::PwmMultiChannel;
use flowmbed_peripherals::sensors::traits::AnalogReaderMultiChannel;
use flowmbed_peripherals::sensors::{DS18B20Array, DS18B20Resolution};

use flowmbed_esp32::hal;

#[allow(unused_imports)]
use log::*;


fn main() {
  esp_idf_svc::log::EspLogger::initialize_default();

  let internal_peripherals = Peripherals::take().unwrap();
  let mut analog_ch_0: hal::AnalogChannel<'_, _, adc::Atten11dB<_>> = hal::AnalogChannel::new(
    internal_peripherals.pins.gpio32).unwrap();

  let mut adc_in = hal::AnalogReaderMultiChannel {
    driver: adc::AdcDriver::new(
      internal_peripherals.adc1,
      &adc::config::Config::new().calibration(true)
    ).unwrap(),
    channels: [
      &mut analog_ch_0
    ]
  };

  let mut pwm_out = 
    hal::PwmMultiChannel::<1>::builder(units::Hertz(1000), internal_peripherals.ledc.timer0).unwrap()
      .add_channel(internal_peripherals.ledc.channel0, internal_peripherals.pins.gpio16).unwrap()
      .build().unwrap();

  let mut target_speed = 0.0;

  loop {
    let mut change = adc_in.read_channel(0).unwrap() - 1.697;
    if (-0.1 < change) && (change < 0.1) {
      change = 0.0;
    }
    target_speed += change * 0.002;
    
    if target_speed <  0.0 {
      target_speed = 0.0;
    } else if target_speed > 1.0 {
      target_speed = 1.0;
    }
    // target_speed = 0.5;
    pwm_out.channel(0).unwrap().enable();
    pwm_out.channel(0).unwrap().set_duty(target_speed);

    println!("{}", target_speed);
    std::thread::sleep(std::time::Duration::from_millis(10));
  }

  // // Scala
  // val x: Double = 6.0;
  // // Rust
  // let x: f64 = 6.0;

}