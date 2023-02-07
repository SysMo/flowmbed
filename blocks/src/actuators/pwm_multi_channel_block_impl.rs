use flowmbed_dynsys::core as ds_core;

use super::pwm_multi_channel_block_auto::*;

/// Implementation DynamicalSystem protocol
#[allow(unused_variables)]
impl<'a, const N: usize> ds_core::DynamicalSystem<'a> for PwmMultiChannelBlock<'a, N> {
  fn init(&mut self) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::init
    for i in 0..N {
      self.periph_out.channel(i)?.set_duty(self.duty[i])?
    }
    Ok(())
    // >>> End section @DynamicalSystem::init
  }

  fn step(&mut self, ssi: &ds_core::SystemStateInfo) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::step
    for i in 0..N {
      self.periph_out.channel(i)?.set_duty(self.duty[i])?
    }
    Ok(())
    // >>> End section @DynamicalSystem::step
  }
}

// >>> Begin section @Begin section @Helpers
// >>> End section @Begin section @Helpers
