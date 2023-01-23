use flowmbed_dynsys::core as dscore;

use super::pwm_output_auto::*;

/// Implementation DynamicalSystem protocol
#[allow(unused_variables)]
impl<'a> dscore::DynamicalSystem for PWMOutput<'a> {
  fn init(&mut self) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::init
    Aux::apply(self, *self.duty)
    // >>> End section @DynamicalSystem::init

  }

  fn step(&mut self, ssi: &dscore::SystemStateInfo) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::step
    Aux::apply(self, *self.duty)
    // >>> End section @DynamicalSystem::step
  }
}

// >>> Begin section @Begin section @Helpers
struct Aux;
impl Aux {
  fn apply<'a>(block: &mut PWMOutput<'a>, duty: dscore::Float) -> anyhow::Result<()> {
    // use log::info;
    // let duty_i = (duty_f * (block.out.get_max_duty() as f64)).round() as u32;
    // info!("duty: {}", duty_i);
    block.output.set_duty(duty)
  }
}
// >>> End section @Begin section @Helpers
