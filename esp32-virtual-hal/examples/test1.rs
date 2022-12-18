extern crate esp32_virtual_hal;

use embedded_hal::digital::OutputPin;
use esp32_virtual_hal::{peripherals::Peripherals, gpio::Gpio2};
// use esp32_virtual_hal::gpio::Pin

type DO<'a> = &'a mut dyn OutputPin<Error = anyhow::Error>;

struct System<'a> {
  led1: DO<'a>,
  led2: DO<'a>
}

impl<'a> System<'a> {
  pub fn new(peripherals: &'a mut Peripherals) -> System<'a> {
    System { 
      led1: &mut peripherals.pins.gpio2,
      led2: &mut peripherals.pins.gpio4 
    }
  }

  pub fn blink(&mut self) {
    self.led1.set_low();
    self.led1.set_high();

    self.led2.set_low();
    self.led2.set_high();
  }
}



pub fn main() -> anyhow::Result<()> {
  let mut peripherals = Peripherals::take()?;
  let mut system = System::new(&mut peripherals);
  system.blink();
  Ok(())
}