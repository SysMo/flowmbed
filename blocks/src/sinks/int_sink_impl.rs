use flowmbed_dynsys::core as dscore;

use super::int_sink_auto::*;

/// Implementation DynamicalSystem protocol
#[allow(unused_variables)]
impl<'a> dscore::DynamicalSystem<'a> for IntSink<'a> {
  fn init(&mut self) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::init
    self.sink.send(Value::Int(*self.input))
    // >>> End section @DynamicalSystem::init
  }

  fn step(&mut self, ssi: &dscore::SystemStateInfo) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::step
    self.sink.send(Value::Int(*self.input))
    // >>> End section @DynamicalSystem::step
  }
}

// >>> Begin section @Begin section @Helpers
use flowmbed_dynsys::data::Value;
// >>> End section @Begin section @Helpers
