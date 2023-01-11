use flowmbed_dynsys::core as dscore;

use super::digital_output_auto::*;

#[doc=" Implementation"]
#[allow(unused_variables)]
impl<'a> dscore::DynamicalSystem for DigitalOutput<'a> {
  fn init(&mut self) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::init
    self.current.initialize(*self.input);
    Aux::apply(self, *self.input)?;
    Ok(())
    // >>> End section @DynamicalSystem::init
  }

  fn step(&mut self, ssi: &dscore::SystemStateInfo) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::step
    if *self.input != *self.current {
      self.current.update(*self.input, ssi);
      Aux::apply(self, *self.input)?;
    }
    Ok(())
    // >>> End section @DynamicalSystem::step
  }
}

// >>> Begin section @Helpers
struct Aux;
impl Aux {
  fn apply(block: &mut DigitalOutput, value: bool) -> anyhow::Result<()> {
    if value {
      block.out.set_high()?;
    } else {
      block.out.set_low()?;
    }
    Ok(())
  }
}
// >>> End section @Helpers