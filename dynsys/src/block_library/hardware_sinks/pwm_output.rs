use crate::core::{
  Block,
  SystemStateInfo, StorageSize, DefaultSystemStrorage,
  SystemStorageBuilder,
  Parameter, DiscreteState, Output, Input,
};

use super::super::hal;

use const_default::ConstDefault;

pub struct PwmOutput<'a> {
  pub input: Input<'a, bool>,
  pub current: DiscreteState<'a, bool>,
  // Hardware Output
  out: hal::IOutputPin<'a>,
}

// TODO Implement