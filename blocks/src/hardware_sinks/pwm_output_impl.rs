use flowmbed_dynsys::core as dscore;

use super::pwm_output_auto::*;

/// Implementation DynamicalSystem protocol
#[allow(unused_variables)]
impl<'a> dscore::DynamicalSystem for PWMOutput<'a> {
  fn init(&mut self) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::init
    Aux::apply(self, *self.duty);
    Ok(())
    // >>> End section @DynamicalSystem::init

  }

  fn step(&mut self, ssi: &dscore::SystemStateInfo) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::step
    Aux::apply(self, *self.duty);
    Ok(())
    // >>> End section @DynamicalSystem::step
  }
}

// >>> Begin section @Begin section @Helpers
struct Aux;
impl Aux {
  fn apply<'a>(block: &mut PWMOutput<'a>, duty_f: f64) {
    // use log::info;
    let duty_i = (duty_f * (block.out.get_max_duty() as f64)).round() as u32;
    // info!("duty: {}", duty_i);
    block.out.set_duty(duty_i);
  }
}
// >>> End section @Begin section @Helpers
