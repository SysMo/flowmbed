use const_default::ConstDefault;
use const_default_derive::ConstDefault;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio;
use esp_idf_hal::adc;
use flowmbed_peripherals::actuators::traits::PwmMultiChannel;
use flowmbed_peripherals::sensors::traits::AnalogReaderMultiChannel;
use flowmbed_peripherals::sensors::{DS18B20Array, DS18B20Resolution};
use flowmbed_peripherals::sensors::traits::{AnalogReader};
use flowmbed_dynsys::{util::containers::RefOnce};
use flowmbed_esp32::hal;
use lazy_static::lazy_static;
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

#[derive(ConstDefault)]
struct PeripheralsMultichannelAdc<'a> {
  pub adc_channel1: RefOnce<hal::AnalogChannel<'a, gpio::Gpio32, adc::Atten11dB<adc::ADC1>>>,
  pub adc_channel2: RefOnce<hal::AnalogChannel<'a, gpio::Gpio33, adc::Atten11dB<adc::ADC1>>>,
  pub adc_reader: RefOnce<hal::AnalogReaderMultiChannel<'a, adc::ADC1, 2>>,
}


fn init_multichannel_adc() -> anyhow::Result<&'static PeripheralsMultichannelAdc<'static>> {
  static MCU_PERIPHERALS: PeripheralsMultichannelAdc<'static> = PeripheralsMultichannelAdc::DEFAULT;

  let p = Peripherals::take().unwrap();

  MCU_PERIPHERALS.adc_channel1.init(
    hal::AnalogChannel::new(p.pins.gpio32)?
  )?;
  MCU_PERIPHERALS.adc_channel2.init(
    hal::AnalogChannel::new(p.pins.gpio33)?
  )?;
  
  let adc_channel_config = adc::config::Config::new()
    .calibration(true);
  
  MCU_PERIPHERALS.adc_reader.init(
    hal::AnalogReaderMultiChannel {
      driver: adc::AdcDriver::new(
        p.adc1,
        &adc_channel_config
      )?,
      channels: [
        MCU_PERIPHERALS.adc_channel1.mut_ref()?,
        MCU_PERIPHERALS.adc_channel2.mut_ref()?,
      ]
    }
  )?;

  Ok(&MCU_PERIPHERALS)
}

fn test_multichannel_adc<'a>(p: &PeripheralsMultichannelAdc) -> anyhow::Result<()> {
  loop {
    let values = p.adc_reader.mut_ref()?.read_all()?;
    info!("{:?}", values);
    std::thread::sleep(std::time::Duration::from_millis(100));
  }
  Ok(())
}


fn test_pwm() -> anyhow::Result<()> {
  use esp_idf_hal::ledc;
  use esp_idf_hal::prelude::*;

  #[derive(ConstDefault)]
  struct PeripheralsMultichannelPwm<'a> {
    pwm1: RefOnce<hal::PwmMultiChannel<'a, 2>>,
  }  

  static MCU_PERIPHERALS: PeripheralsMultichannelPwm = PeripheralsMultichannelPwm::DEFAULT;

  let peripherals = Peripherals::take().unwrap();
  MCU_PERIPHERALS.pwm1.init( 
    hal::PwmMultiChannel::new(1000_u32.Hz(), peripherals.ledc.timer0)?
      .add_channel(peripherals.ledc.channel0, peripherals.pins.gpio16)?
      .add_channel(peripherals.ledc.channel1, peripherals.pins.gpio17)?
  )?;

  let mut pwm1 = MCU_PERIPHERALS.pwm1.mut_ref()?;
  pwm1.channel(0)?.set_duty(1.0)?;
  pwm1.channel(1)?.set_duty(0.03)?;
  std::thread::sleep(std::time::Duration::from_secs(5));

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


  // let p = init_multichannel_adc()?;
  // test_multichannel_adc(p)?;
  // test_multichannel_adc();
  test_pwm()?;
  // test_ds18b20();

  
  Ok(())
}