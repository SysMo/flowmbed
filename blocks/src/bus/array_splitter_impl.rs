use flowmbed_dynsys::core as ds_core;

use super::array_splitter_auto::*;

/// Implementation DynamicalSystem protocol
#[allow(unused_variables)]
impl<'a, T: Copy + Default, const N: usize> ds_core::DynamicalSystem<'a> for ArraySplitter<'a, T, N> {
  fn init(&mut self) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::init
    for i in 0..N {
      self.outputs[i].initialize((*self.input)[i]);
    }
    Ok(())
    // >>> End section @DynamicalSystem::init
  }

  fn step(&mut self, ssi: &ds_core::SystemStateInfo) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::step
    for i in 0..N {
      self.outputs[i].update((*self.input)[i], ssi);
    }
    Ok(())
    // >>> End section @DynamicalSystem::step
  }
}

// >>> Begin section @Begin section @Helpers
// >>> End section @Begin section @Helpers
