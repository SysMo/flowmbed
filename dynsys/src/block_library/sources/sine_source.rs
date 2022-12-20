use crate::core::{
  Block,
  SystemStateInfo, StorageSize, DefaultSystemStrorage,
  SystemStorageBuilder,
  Parameter, DiscreteState, Output,
};
use const_default::ConstDefault;

pub struct SineSource<'a> {
  pub period: Parameter<'a, f64>,
  pub initial: Parameter<'a, bool>,
  pub output: Output<'a, f64>,
  current: DiscreteState<'a, bool>,
  last_change: DiscreteState<'a, f64>,
}

// TODO Implement