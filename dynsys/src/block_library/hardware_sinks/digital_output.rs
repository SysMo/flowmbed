use crate::core::{
  Block,
  SystemStateInfo, StorageSize, DefaultSystemStrorage,
  SystemStorageBuilder,
  Parameter, DiscreteState, Output, Input,
};

use super::super::hal;

use const_default::ConstDefault;

pub struct DigitalOutput<'a> {
  pub input: Input<'a, bool>,
  pub current: DiscreteState<'a, bool>,
  // Hardware Output
  out: hal::IOutputPin<'a>,
}

impl<'a> DigitalOutput<'a>
{
  pub fn new<ST: DefaultSystemStrorage>(
    builder: &mut SystemStorageBuilder<'a, ST>,
    out: hal::IOutputPin<'a>,
  ) -> DigitalOutput<'a> {
    DigitalOutput {
      input: builder.create_input(), 
      current: builder.create_discrete_state(false),
      out: out
    }
  }

  pub fn init(&mut self) -> anyhow::Result<()> 
  {
    self.current.initialize(*self.input);
    self.apply(*self.input)?;
    Ok(())
  }

  pub fn compute(&mut self, ssi: &SystemStateInfo) -> anyhow::Result<()> 
  {
    if *self.input != *self.current {
      self.current.update(*self.input, ssi);
      self.apply(*self.input)?;
    }
    Ok(())
  }

  fn apply(&mut self, value: bool) -> anyhow::Result<()> {
    if value {
      self.out.set_high()?;
    } else {
      self.out.set_low()?;
    }
    Ok(())
  }

}

impl<'a> Block for DigitalOutput<'a> {
  const BLOCK_SIZE: StorageSize = StorageSize {
    b_dstate: 1,
    ..StorageSize::DEFAULT
  };
}
