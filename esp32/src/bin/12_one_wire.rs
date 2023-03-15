use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio;
use esp_idf_hal::delay::Ets;
use flowmbed_peripherals::sensors::{DS18B20Array, DS18B20Resolution};
use flowmbed_peripherals::sensors::traits::AnalogReaderMultiChannel;
#[allow(unused_imports)]
use log::*;


fn test_ds18b20() -> anyhow::Result<()> {
  let peripherals = Peripherals::take().unwrap();
  // DS18B20  
  let pin = gpio::PinDriver::input_output_od(peripherals.pins.gpio4)?;
  let mut one = DS18B20Array::<_, _, _, 2>::new(pin, Ets)?;
  one.set_resolution(DS18B20Resolution::Bits12)?;

  loop {
    // println!("{}", 0.0);
    // println!("{:?}", 0.0);
    const f_1_16: f32 = 1.0 / 16.0;
    let x = 0.9375;
    // println!("{} {} {}", x, x - f_1_16, x - f_1_16 - f_1_16);
    println!("Tick");
    let readings = one.read_all()?;
    let out = readings.iter().enumerate()
      .map(|(i, x)| format!("T{i} = {x}"))
      .collect::<Vec<_>>()
      .join("; ");

    // println!("Temperatures: {}", out);
    std::thread::sleep(std::time::Duration::from_millis(10));
  }

  Ok(())
}


fn main() -> anyhow::Result<()> {
  esp_idf_svc::log::EspLogger::initialize_default();
  test_ds18b20()?;
  Ok(())
}