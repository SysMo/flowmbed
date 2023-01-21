use flowmbed_peripherals::sensors::traits::OneShotAnalog;
use flowmbed_dynsys::core::Float;
use esp_idf_hal::adc;
use esp_idf_hal::gpio;

pub struct ADCReader<'a, D: adc::Adc, P: gpio::ADCPin, Attn> 
{
  pub driver: adc::AdcDriver<'a, D>,
  pub channel: adc::AdcChannelDriver<'a, P, Attn>
}

impl<'a, D: adc::Adc, P: gpio::ADCPin, Attn> OneShotAnalog 
for ADCReader<'a, D, P, Attn> 
where Attn: adc::Attenuation<<P as gpio::ADCPin>::Adc> {
  fn read(&mut self) -> Result<Float, anyhow::Error> {
    self.driver.read(&mut self.channel)
      .map(|x| x as Float)
      .map_err(|e| e.into())
  }
}