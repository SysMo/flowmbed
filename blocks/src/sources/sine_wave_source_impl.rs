use flowmbed_dynsys::core as dscore;

use super::sine_wave_source_auto::*;

#[doc=" Implementation"]
#[allow(unused_variables)]
impl<'a> dscore::DynamicalSystem for SineWaveSource<'a> {
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

  pub fn compute_output(block: &SineWaveSource, t: f64) -> f64 {
    let x: f64 = (t / *block.period + *block.phase) * 2.0 * PI;
    *block.amplitude * x.sin()
  }
}
// >>> End section @Helpers