use crate::dynsys::{SystemStateInfo, StorageSize};
use crate::dynsys::system_storage::{SystemStorageBuilder, DefaultSystemStrorage};
use crate::dynsys::variables::{DiscreteState, Input};
use embedded_hal::digital::OutputPin;
use std::cell::RefMut;
use crate::dynsys::Block;
use const_default::ConstDefault;

type DO<'a> = RefMut<'a, Box<dyn OutputPin<Error = anyhow::Error>>>;

pub struct DigitalOutput<'a> {
  pub input: Input<'a, bool>,
  pub current: DiscreteState<'a, bool>,
  // Periphery  
  pub digital_output: DO<'a>
}

impl<'a> DigitalOutput<'a> {
  pub fn new<ST: DefaultSystemStrorage>(
    builder: &mut SystemStorageBuilder<'a, ST>, 
    digital_output: DO<'a>
  ) -> DigitalOutput<'a> {
    DigitalOutput {
      input: builder.create_input(), 
      current: builder.create_discrete_state(false),
      digital_output: digital_output
    }
  }

  pub fn init(&mut self) {
  }

  pub fn compute(&mut self, _ssi: &SystemStateInfo) -> anyhow::Result<()> {
    if *self.input != *self.current {
      self.current.update(*self.input);
      if *self.input {
        self.digital_output.set_high()?;
      } else {
        self.digital_output.set_low()?;
      }
    }
    Ok(())
  }
}

impl<'a> Block for DigitalOutput<'a> {
  const size: StorageSize = StorageSize {
    b_dstate: 1,
    ..StorageSize::DEFAULT
  };
}
