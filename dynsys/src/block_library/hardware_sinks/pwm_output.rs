use crate::core::{
  Block,
  SystemStateInfo, StorageSize, DefaultSystemStrorage,
  SystemStorageBuilder,
  Parameter, DiscreteState, Output, Input, RequiresStorage,
};

use super::super::hal;

use const_default::ConstDefault;

pub struct PwmOutput<'a> {
  // pub freq: Parameter<'a, f64>,
  pub input: Input<'a, f64>,
  // Hardware Output
  out: hal::PwmPin<'a>,
}

impl<'a> PwmOutput<'a>
{
  pub fn new<ST: DefaultSystemStrorage>(
    builder: &mut SystemStorageBuilder<'a, ST>,
    out: hal::PwmPin<'a>,
  ) -> PwmOutput<'a> {
    PwmOutput {
      // freq: builder.create_param(1e3),
      input: builder.create_input(),
      out: out
    }
  }

  pub fn init(&mut self) -> anyhow::Result<()> 
  {
    self.apply(*self.input)?;
    Ok(())
  }

  pub fn compute(&mut self, ssi: &SystemStateInfo) -> anyhow::Result<()> 
  {
    self.apply(*self.input)?;
    Ok(())
  }

  fn apply(&mut self, value: f64) -> anyhow::Result<()> {
    // let max_duty = self.out.get_max_duty();
    // let duty = (max_duty * *self.input).round();
    self.out.set_duty(value);
    Ok(())
  }

}

impl<'a> RequiresStorage for PwmOutput<'a> {
  const SIZE: StorageSize = StorageSize {
    // b_dstate: 1,
    // r_param: 1,
    ..StorageSize::DEFAULT
  };
}