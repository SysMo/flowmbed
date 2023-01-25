use flowmbed_peripherals::sensors::traits as sensors;
use flowmbed_peripherals::actuators::traits as actuators;
use flowmbed_dynsys::core::Bool;
use esp_idf_hal::adc;
use esp_idf_hal::gpio;

pub struct DigitalInputPin<'a, P: gpio::InputPin> (
  pub gpio::PinDriver<'a, P, gpio::Input>
);

impl<'a, P: gpio::InputPin> sensors::DigitalReader for DigitalInputPin<'a, P> {
  fn read(&mut self) -> anyhow::Result<Bool> {
    Ok(self.0.is_high())
  }
}

impl<'a, P: gpio::InputPin> From<gpio::PinDriver<'a, P, gpio::Input>> for DigitalInputPin<'a, P> {
  fn from(x: gpio::PinDriver<'a, P, gpio::Input>) -> Self {
    DigitalInputPin(x)
  }
}

pub struct DigitalOutputPin<'a, P: gpio::OutputPin> (
  pub gpio::PinDriver<'a, P, gpio::Output>
);

impl<'a, P: gpio::OutputPin> actuators::DigitalOutputPin for DigitalOutputPin<'a, P> {
  fn set_low(&mut self) -> anyhow::Result<()> {
    Ok(self.0.set_low()?)
  }

  fn set_high(&mut self) -> anyhow::Result<()> {
    Ok(self.0.set_high()?)
  }
}

impl<'a, P: gpio::OutputPin> From<gpio::PinDriver<'a, P, gpio::Output>> for DigitalOutputPin<'a, P> {
  fn from(x: gpio::PinDriver<'a, P, gpio::Output>) -> Self {
    DigitalOutputPin(x)
  }
}
