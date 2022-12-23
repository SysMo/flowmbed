use std::f64::consts::PI;

use crate::core::{
  Block,
  SystemStateInfo, StorageSize, DefaultSystemStrorage,
  SystemStorageBuilder,
  Parameter, DiscreteState, Output, RequiresStorage,
};
use const_default::ConstDefault;
use crate::util::block_macros::*;

pub struct SineSource<'a> {
  pub period: Parameter<'a, f64>,
  pub phase: Parameter<'a, f64>,
  pub amplitude: Parameter<'a, f64>,
  
  pub output: Output<'a, f64>,
}

block_builder!(SineSource, 
  [period, Parameter, f64]
  [phase, Parameter, f64]
  [amplitude, Parameter, f64]
);

impl<'a> SineSource<'a> {
  pub fn new<ST: DefaultSystemStrorage>(
    builder: &mut SystemStorageBuilder<'a, ST>
  ) -> SineSource<'a> {
    SineSource { 
      period: builder.create_param(1.0),
      phase: builder.create_param(0.0),
      amplitude: builder.create_param(1.0),
      output: builder.create_output(0.0),
    }
  }

  pub fn builder<ST: DefaultSystemStrorage>(
    builder: &mut SystemStorageBuilder<'a, ST>
  ) -> Builder<'a> {
    Builder { component: Self::new(builder) }
  }

  pub fn init(&mut self) -> anyhow::Result<()> {
    self.output.initialize(self.compute_output(0.0));
    Ok(())
  }

  pub fn compute(&self, ssi: &SystemStateInfo) -> anyhow::Result<()> {
    self.output.update(self.compute_output(ssi.t), ssi);
    Ok(())
  }

  fn compute_output(&self, t: f64) -> f64 {
    let x: f64 = (t / *self.period + *self.phase) * 2.0 * PI;
    *self.amplitude * x.sin()
  }
}

impl<'a> RequiresStorage for SineSource<'a> {
  const SIZE: StorageSize = StorageSize {
    r_param: 3, r_out: 1,
    ..StorageSize::DEFAULT
  };
}

