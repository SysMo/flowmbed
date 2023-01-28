use flowmbed_dynsys::core as dscore;

use super::analog_reader_multi_channel_block_auto::*;

/// Implementation DynamicalSystem protocol
#[allow(unused_variables)]
impl<'a> dscore::DynamicalSystem for AnalogReaderMultiChannelBlock<'a> {
  fn init(&mut self) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::init
    let reading = self.sensor.read_all()?;
    self.output1.initialize(reading[0]);
    self.output2.initialize(reading[1]);
    Ok(())
    // >>> End section @DynamicalSystem::init
  }

  fn step(&mut self, ssi: &dscore::SystemStateInfo) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::step
    let reading = self.sensor.read_all()?;
    self.output1.update(reading[0], ssi);
    self.output2.update(reading[1], ssi);
    Ok(())
    // >>> End section @DynamicalSystem::step
  }
}

// >>> Begin section @Begin section @Helpers
// >>> End section @Begin section @Helpers
