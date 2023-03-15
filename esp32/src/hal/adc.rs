use flowmbed_peripherals::sensors::traits as sensors;
use flowmbed_dynsys::core::{Float, DynRefMut};
use esp_idf_hal::adc;
use esp_idf_hal::gpio;

pub struct ADCReader<'a, D: adc::Adc, P: gpio::ADCPin, Attn> 
{
  pub driver: adc::AdcDriver<'a, D>,
  pub channel: adc::AdcChannelDriver<'a, P, Attn>
}

impl<'a, D: adc::Adc, P: gpio::ADCPin, Attn> sensors::AnalogReader 
for ADCReader<'a, D, P, Attn> 
where Attn: adc::Attenuation<<P as gpio::ADCPin>::Adc> {
  fn read(&mut self) -> Result<Float, anyhow::Error> {
    self.driver.read(&mut self.channel)
      .map(|x| x as Float)
      .map_err(|e| e.into())
  }
}

pub trait Esp32AnalogChannelReader<'a, D: adc::Adc> {
  fn read(&mut self, driver: &mut adc::AdcDriver<'a, D>) -> anyhow::Result<Float>;
}

pub struct Esp32AnalogChannel<'a, P: gpio::ADCPin, Attn> {
  pub channel: adc::AdcChannelDriver<'a, P, Attn>
}

impl<'a, P: gpio::ADCPin, Attn> Esp32AnalogChannel<'a, P, Attn>
where Attn: adc::Attenuation<<P as gpio::ADCPin>::Adc>
{
  pub fn new(pin: P) -> anyhow::Result<Self> {
    Ok(Esp32AnalogChannel {
      channel: adc::AdcChannelDriver::new(pin)?
    })
  }
}


impl<'a, D: adc::Adc, P: gpio::ADCPin, Attn> Esp32AnalogChannelReader<'a, D>
for Esp32AnalogChannel<'a, P, Attn> 
where Attn: adc::Attenuation<<P as gpio::ADCPin>::Adc> {
  fn read(&mut self, driver: &mut adc::AdcDriver<'a, D>) -> Result<Float, anyhow::Error> {
    driver.read(&mut self.channel)
      .map(|x| (x as Float) / 1000.0 )
      .map_err(|e| e.into())
  }
}

pub struct Esp32AnalogReaderMultiChannel<'a, D: adc::Adc, const N: usize>
{
  pub driver: adc::AdcDriver<'a, D>,
  pub channels: [&'a mut dyn Esp32AnalogChannelReader<'a, D>; N]
}

impl<'a, D: adc::Adc, const N: usize> sensors::AnalogReaderMultiChannel<N>
for Esp32AnalogReaderMultiChannel<'a, D, N> {
  fn read_channel(&mut self, id: usize) -> anyhow::Result<f32> {
    self.channels[id].read(&mut self.driver)
  }
}

