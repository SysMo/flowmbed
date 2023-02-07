use flowmbed_dynsys::core as ds_core;

use super::array_sink_auto::*;

/// Implementation DynamicalSystem protocol
#[allow(unused_variables)]
impl<'a, T: Copy + std::fmt::Display, const N: usize> ds_core::DynamicalSystem<'a> for ArraySink<'a, T, N> {
  fn init(&mut self) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::init
    Ok(())
    // >>> End section @DynamicalSystem::init
  }

  fn step(&mut self, ssi: &ds_core::SystemStateInfo) -> anyhow::Result<()> {
    // >>> Begin section @DynamicalSystem::step
    use flowmbed_dynsys::data::Value;
    // >>> Begin section @DynamicalSystem::step
    let output = (*self.input).iter().map(|x| format!("{}", x))
      .collect::<Vec<_>>().join(", ");
    self.sink.send(Value::String(output))?;
    Ok(())
    // >>> End section @DynamicalSystem::step
  }
}

// >>> Begin section @Begin section @Helpers
// struct Aux;
// impl Aux {

// }
// >>> End section @Begin section @Helpers
