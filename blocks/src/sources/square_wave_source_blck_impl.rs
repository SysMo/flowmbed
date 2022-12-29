use flowmbed_dynsys::core as dscore;

use super::square_wave_source_blck_auto::*;

#[doc=" Implementation"]
#[allow(unused_variables)]
impl<'a> dscore::DynamicalSystem for SquareWaveSource<'a> {
  fn init(&mut self) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::init
    self.current.initialize(*self.initial);
    self.output.initialize(*self.initial);
    Ok(())
    // >>> End section @DynamicalSystem::init
  }

  fn step(&mut self, ssi: &dscore::SystemStateInfo) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::step
    let tau_on = *self.period *  *self.duty;
    let tau_off = *self.period - tau_on;
    // Update state
    if *self.current && (ssi.t - *self.last_change >= tau_on) {
      self.current.update(false, ssi);
      self.last_change.update(ssi.t, ssi);
    } else if !*self.current && (ssi.t - *self.last_change >= tau_off) {
      self.current.update(true, ssi);
      self.last_change.update(ssi.t, ssi);
    }
    // Update outputs
    self.output.update(*self.current, ssi);
    Ok(())
    // >>> End section @DynamicalSystem::step
  }
}
