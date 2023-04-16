use esp_idf_hal::i2c::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;


fn test_ads11x5() -> anyhow::Result<()> {
  use ads1x1x::{channel, Ads1x1x, SlaveAddr, FullScaleRange};
  use embedded_hal_0_2::adc::OneShot;
  use nb::block;

  let peripherals = Peripherals::take().unwrap();
  let i2c = peripherals.i2c0;
  let sda = peripherals.pins.gpio21;
  let scl = peripherals.pins.gpio22;

  let config = I2cConfig::new().baudrate(100.kHz().into());
  let i2c = I2cDriver::new(i2c, sda, scl, &config)?;


  let address = SlaveAddr::default();
  let mut adc = Ads1x1x::new_ads1115(i2c, address);
  adc.set_full_scale_range(FullScaleRange::Within4_096V).unwrap();
  // adc.set_data_rate(rate)
  // adc.
  loop {
    let values = [
      block!(adc.read(&mut channel::SingleA0)).unwrap(),
      block!(adc.read(&mut channel::SingleA1)).unwrap(),
      block!(adc.read(&mut channel::SingleA2)).unwrap(),
      block!(adc.read(&mut channel::SingleA3)).unwrap(),
    ];
    
    const V_CONV: f32 = 4.096 / 32768.0;
    let voltages: [f32; 4] = core::array::from_fn(|i| 
      (values[i] as f32) * V_CONV
    ) ;

    println!("Ph Voltage: {}", voltages[2]);

  }
  
  // get I2C device back
  let _dev = adc.destroy_ads1115();

  Ok(())
}

fn test_ads11x5_2() -> anyhow::Result<()> {
  use flowmbed_peripherals::sensors::ads1x1x;
  use flowmbed_peripherals::sensors::traits::{AnalogReader, AnalogReaderMultiChannel};
  let peripherals = Peripherals::take().unwrap();
  let i2c = peripherals.i2c0;
  let sda = peripherals.pins.gpio21;
  let scl = peripherals.pins.gpio22;

  let config = I2cConfig::new().baudrate(100.kHz().into());
  let mut i2c = I2cDriver::new(i2c, sda, scl, &config)?;

  let mut ads1115 = ads1x1x::Ads1x1xDeviceConfigurator::new(
    i2c, ads1x1x::FullScaleRange::Within4_096V
  ).add_channel(ads1x1x::ChannelSelection::SingleA0)
    .add_channel(ads1x1x::ChannelSelection::SingleA1)
    .add_channel(ads1x1x::ChannelSelection::DifferentialA2A3)
    .build();

  // ads1x1x::ChannelSelection::DifferentialA0A1
  println!("{:?}", ads1115.read_all().unwrap());
  let mut channels = ads1115.split();
  let diff_ch: &mut dyn AnalogReader = &mut channels[2];
  println!("{}", diff_ch.read().unwrap());
  Ok(())
}


fn main() -> anyhow::Result<()> {
  esp_idf_svc::log::EspLogger::initialize_default();


  // test_ads11x5()?;
  test_ads11x5_2()?;
  Ok(())
}