use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio;
use flowmbed_esp32::peripherals::{DS18B20Array, ds18b20_array};

fn main() -> anyhow::Result<()> {
  esp_idf_svc::log::EspLogger::initialize_default();

  let peripherals = Peripherals::take().unwrap();
  let mut pin = gpio::PinDriver::input_output_od(peripherals.pins.gpio4)?;
  let mut one = DS18B20Array::new(pin, Ets)?;

  one.set_resolution(ds18b20_array::Resolution::Bits9);

  loop {
    one.get_temperature();
    std::thread::sleep(
      std::time::Duration::from_millis(1000)
    )
  }
  
  Ok(())
}