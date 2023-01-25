use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio;
use esp_idf_hal::adc;
use flowmbed_peripherals::sensors::traits::AnalogReaderMultiChannel;
use flowmbed_peripherals::sensors::{DS18B20Array, DS18B20Resolution};
use flowmbed_peripherals::sensors::traits::{OneShotAnalog};
use flowmbed_esp32::hal;
use log::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

fn test_adc() -> anyhow::Result<()> {
  let peripherals = Peripherals::take().unwrap();
  let mut reader: hal::ADCReader<'_, _, _, adc::Atten0dB<_>> = hal::ADCReader {
    driver: adc::AdcDriver::new(
      peripherals.adc2, 
      &adc::config::Config::new()
        .calibration(true)
        .resolution(adc::config::Resolution::Resolution11Bit)
    )?,
    channel: adc::AdcChannelDriver::new(peripherals.pins.gpio12)?
  };

  // println!("{}", reader.read()?);
  Ok(())
}

fn get_field_ref<'a, T>(opt: &'a mut Option<T>) -> anyhow::Result<&'a mut T> {
  opt.as_mut().ok_or_else(|| anyhow::anyhow!("Empty field"))
}

struct Peripherals_Multichannel<'a> {
  adc_reader: hal::AnalogReaderMultiChannel<'a, adc::ADC1, 2>,
}

fn init_multichannel_adc() -> anyhow::Result<()> {
  let peripherals = Peripherals::take().unwrap();

  let mut adc_channel1: hal::AnalogChannel<gpio::Gpio32, adc::Atten11dB<adc::ADC1>> = 
    hal::AnalogChannel::new(peripherals.pins.gpio32)?;
  let mut adc_channel2: hal::AnalogChannel<gpio::Gpio33, adc::Atten11dB<adc::ADC1>> = 
    hal::AnalogChannel::new(peripherals.pins.gpio33)?;
  let adc_channel_config = adc::config::Config::new()
    .calibration(true);
  
  let mut p = Peripherals_Multichannel {
    adc_reader: hal::AnalogReaderMultiChannel {
      driver: adc::AdcDriver::new(
        peripherals.adc1,
        &adc_channel_config
      )?,
      channels: [
        &mut adc_channel1,
        &mut adc_channel2,
      ]
  }};

  loop {
    let values = p.adc_reader.read_all()?;
    info!("{:?}", values);
    std::thread::sleep(std::time::Duration::from_millis(100));
  }

  Ok(())
}

// fn test_multichannel_adc<'a>() -> anyhow::Result<Peripherals_Multichannel<'a>> {

// }

fn test_ds18b20() -> anyhow::Result<()> {
  let peripherals = Peripherals::take().unwrap();
  // // DS18B20  
  // let mut pin = gpio::PinDriver::input_output_od(peripherals.pins.gpio4)?;
  // let mut one = DS18B20Array::new(pin, Ets)?;
  // one.set_resolution(DS18B20Resolution::Bits9)?;
  // println!("{}", one.get_temperature()?);

  Ok(())
}

fn main() -> anyhow::Result<()> {
  esp_idf_svc::log::EspLogger::initialize_default();

  // test_adc();
  init_multichannel_adc();
  // test_multichannel_adc();
  // test_ds18b20();

  
  Ok(())
}