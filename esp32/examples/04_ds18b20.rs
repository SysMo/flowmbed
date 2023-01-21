use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio;
use esp_idf_hal::adc;
use flowmbed_peripherals::sensors::{DS18B20Array, DS18B20Resolution};
use flowmbed_peripherals::sensors::traits::{OneShotAnalog};
use flowmbed_esp32::hal::ADCReader;

fn main() -> anyhow::Result<()> {
  esp_idf_svc::log::EspLogger::initialize_default();

  let peripherals = Peripherals::take().unwrap();
  
  //ADC Input
  // let mut adc_driver = adc::AdcDriver::new(
  //   peripherals.adc2, 
  //   &adc::config::Config::new().calibration(true)
  // )?;
  // let mut adc_pin: adc::AdcChannelDriver<'_, gpio::Gpio12, adc::Atten11dB<_>> =
  //   adc::AdcChannelDriver::new(peripherals.pins.gpio12)?;
  // println!("{}", adc_driver.read(&mut adc_pin)?);

  let mut reader: ADCReader<'_, _, _, adc::Atten0dB<_>> = ADCReader {
    driver: adc::AdcDriver::new(
      peripherals.adc2, 
      &adc::config::Config::new()
        .calibration(true)
        .resolution(adc::config::Resolution::Resolution11Bit)
    )?,
    channel: adc::AdcChannelDriver::new(peripherals.pins.gpio12)?
  };

  println!("{}", reader.read()?);

  // DS18B20  
  let mut pin = gpio::PinDriver::input_output_od(peripherals.pins.gpio4)?;
  let mut one = DS18B20Array::new(pin, Ets)?;
  one.set_resolution(DS18B20Resolution::Bits9)?;
  println!("{}", one.get_temperature()?);

  
  Ok(())
}