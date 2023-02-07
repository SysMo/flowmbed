use flowmbed_dynsys::core as dscore;

use super::digital_reader_block_auto::*;

/// Implementation DynamicalSystem protocol
#[allow(unused_variables)]
impl<'a> dscore::DynamicalSystem<'a> for DigitalReaderBlock<'a> {
  fn init(&mut self) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::init
    self.output.initialize(self.periph_reader.read()?);
    Ok(())
    // >>> End section @DynamicalSystem::init
  }

  fn step(&mut self, ssi: &dscore::SystemStateInfo) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::step
    self.output.update(self.periph_reader.read()?, ssi);
    Ok(())
    // >>> End section @DynamicalSystem::step
  }
}

// >>> Begin section @Begin section @Helpers
// >>> End section @Begin section @Helpers
