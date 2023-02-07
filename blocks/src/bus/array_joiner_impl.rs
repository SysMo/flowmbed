use flowmbed_dynsys::core as ds_core;

use super::array_joiner_auto::*;

/// Implementation DynamicalSystem protocol
#[allow(unused_variables)]
impl<'a, T: Copy + Default, const N: usize> ds_core::DynamicalSystem<'a> for ArrayJoiner<'a, T, N> {
  fn init(&mut self) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::init
    self.output.initialize(core::array::from_fn(|i| *(self.inputs[i])));
    Ok(())
    // >>> End section @DynamicalSystem::init
  }

  fn step(&mut self, ssi: &ds_core::SystemStateInfo) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::step
    self.output.update(
      core::array::from_fn(|i| *(self.inputs[i])), ssi
    );
    Ok(())
    // >>> End section @DynamicalSystem::step
  }
}

// >>> Begin section @Begin section @Helpers
// >>> End section @Begin section @Helpers
