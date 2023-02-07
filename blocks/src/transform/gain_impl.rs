use flowmbed_dynsys::core as ds_core;

use super::gain_auto::*;

/// Implementation DynamicalSystem protocol
#[allow(unused_variables)]
impl<'a> ds_core::DynamicalSystem<'a> for Gain<'a> {
  fn init(&mut self) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::init
    self.output.initialize(*self.input * *self.gain);
    Ok(())
    // >>> End section @DynamicalSystem::init
  }

  fn step(&mut self, ssi: &ds_core::SystemStateInfo) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::step
    self.output.update(*self.input * *self.gain, ssi);
    Ok(())
    // >>> End section @DynamicalSystem::step
  }
}

// >>> Begin section @Begin section @Helpers
// >>> End section @Begin section @Helpers
