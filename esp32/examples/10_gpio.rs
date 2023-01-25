use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio;
use esp_idf_hal::adc;
use flowmbed_peripherals::sensors::traits::AnalogReaderMultiChannel;
use flowmbed_peripherals::sensors::{DS18B20Array, DS18B20Resolution};
use flowmbed_peripherals::sensors::traits::{AnalogReader};
use flowmbed_esp32::hal;
use log::*;

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

struct PeripheralsMultichannel {
  adc_reader: Box<dyn AnalogReaderMultiChannel<2>>,
}

fn init_multichannel_adc() -> anyhow::Result<PeripheralsMultichannel> {
  let peripherals = Peripherals::take().unwrap();

  let mut adc_channel1: hal::AnalogChannel<gpio::Gpio32, adc::Atten11dB<adc::ADC1>> = 
    hal::AnalogChannel::new(peripherals.pins.gpio32)?;
  let mut adc_channel2: hal::AnalogChannel<gpio::Gpio33, adc::Atten11dB<adc::ADC1>> = 
    hal::AnalogChannel::new(peripherals.pins.gpio33)?;
  let adc_channel_config = adc::config::Config::new()
    .calibration(true);
  
  let mut p = PeripheralsMultichannel {
    adc_reader: Box::new(hal::AnalogReaderMultiChannel {
      driver: adc::AdcDriver::new(
        peripherals.adc1,
        &adc_channel_config
      )?,
      channels: [
        Box::new(adc_channel1),
        Box::new(adc_channel2),
      ]
    })
  };


  Ok(p)
}

fn test_multichannel_adc<'a>(mut p: PeripheralsMultichannel) -> anyhow::Result<()> {
  loop {
    let values = p.adc_reader.read_all()?;
    info!("{:?}", values);
    std::thread::sleep(std::time::Duration::from_millis(100));
  }
  Ok(())
}

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


  let p = init_multichannel_adc()?;
  test_multichannel_adc(p);
  // test_multichannel_adc();
  // test_ds18b20();

  
  Ok(())
}