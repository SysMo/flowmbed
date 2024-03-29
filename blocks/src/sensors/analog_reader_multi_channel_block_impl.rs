use flowmbed_dynsys::core as ds_core;

use super::analog_reader_multi_channel_block_auto::*;

/// Implementation DynamicalSystem protocol
#[allow(unused_variables)]
impl<'a, const N: usize> ds_core::DynamicalSystem<'a> for AnalogReaderMultiChannelBlock<'a, N> {
  fn init(&mut self) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::init
    let readings = self.periph_reader.read_all()?;
    self.readings.initialize(readings);
    Ok(())
    // >>> End section @DynamicalSystem::init
  }

  fn step(&mut self, ssi: &ds_core::SystemStateInfo) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::step
    let readings = self.periph_reader.read_all()?;
    self.readings.update(readings, &ssi);
    Ok(())
    // >>> End section @DynamicalSystem::step
  }
}

// >>> Begin section @Begin section @Helpers
// >>> End section @Begin section @Helpers
