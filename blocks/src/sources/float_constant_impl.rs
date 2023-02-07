use flowmbed_dynsys::core as ds_core;

use super::float_constant_auto::*;

/// Implementation DynamicalSystem protocol
#[allow(unused_variables)]
impl<'a> ds_core::DynamicalSystem<'a> for FloatConstant<'a> {
  fn init(&mut self) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::init
    self.output.initialize(*self.value);
    Ok(())
    // >>> End section @DynamicalSystem::init
  }

  fn step(&mut self, ssi: &ds_core::SystemStateInfo) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::step
    self.output.update(*self.value, ssi);
    Ok(())
    // >>> End section @DynamicalSystem::step
  }
}

// >>> Begin section @Begin section @Helpers
// >>> End section @Begin section @Helpers
