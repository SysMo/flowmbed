use flowmbed_dynsys::core as dscore;

use super::sine_wave_source_auto::*;

/// Implementation DynamicalSystem protocol
#[allow(unused_variables)]
impl<'a> dscore::DynamicalSystem<'a> for SineWaveSource<'a> {
  fn init(&mut self) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::init
    self.output.initialize(
      Helpers::compute_output(self, 0.0)
    );

    Ok(())
    // >>> End section @DynamicalSystem::init
  }

  fn step(&mut self, ssi: &dscore::SystemStateInfo) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::step
    self.output.update(
      Helpers::compute_output(self, ssi.t), ssi
    );    
    Ok(())
    // >>> End section @DynamicalSystem::step
  }
}

// >>> Begin section @Helpers
use std::f64::consts::PI;
struct Helpers;
impl Helpers {
  pub fn compute_output(block: &SineWaveSource, t: dscore::Float) -> dscore::Float {
    let x = (t / *block.period + *block.phase) * 2.0 * (PI as dscore::Float);
    let out = *block.amplitude * x.sin() + *block.offset;
    out
  }

  // pub fn debug(block: &SineWaveSource,  t: f64) {
  //   use log::*;
  //   log::info!("t = {:4e} period = {}, phase = {}, amplitude = {}, offset = {}", t, *block.period, *block.phase, *block.amplitude, *block.offset);
  // }
}
// >>> End section @Helpers