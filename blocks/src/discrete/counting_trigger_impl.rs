use flowmbed_dynsys::core as dscore;

use super::counting_trigger_auto::*;

#[doc=" Implementation"]
#[allow(unused_variables)]
impl<'a> dscore::DynamicalSystem<'a> for CountingTrigger<'a> {
  fn init(&mut self) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::init
    self.current.initialize(*self.initial_state);
    self.output.initialize(*self.initial_state);
    self.counter.initialize(*self.initial_count);
    Ok(())
    // >>> End section @DynamicalSystem::init
  }

  fn step(&mut self, ssi: &dscore::SystemStateInfo) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::step
    if *self.input != *self.last_input {
      if (*self.input && *self.count_on_rising) ||  (!*self.input && !*self.count_on_rising) {
        self.counter.update(*self.counter + 1, ssi);
      }
      self.last_input.update(*self.input, ssi);
    }

    if *self.current && (*self.counter >= *self.pulses_down) {      
      self.counter.update(0, ssi);
      self.current.update(false, ssi);
    } else if !*self.current && (*self.counter >= *self.pulses_up) {
      self.counter.update(0, ssi);
      self.current.update(true, ssi);
    }

    self.output.update(*self.current, ssi);
    Ok(())
    // >>> End section @DynamicalSystem::step
  }
}
