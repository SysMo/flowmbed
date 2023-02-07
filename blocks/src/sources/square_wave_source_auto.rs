use flowmbed_dynsys::core as ds_core;

#[allow(unused_imports)]
use flowmbed_dynsys::core::{Float, Int, Bool, String};

/// Declare the block struct
#[allow(dead_code)]
pub struct SquareWaveSource<'a> {
  // Parameters
  pub period: ds_core::Parameter<'a, ds_core::Float>,
  pub duty: ds_core::Parameter<'a, ds_core::Float>,
  pub initial: ds_core::Parameter<'a, ds_core::Bool>,
  // Inputs
  // Outputs
  pub output: ds_core::Output<ds_core::Bool>,
  // Discrete states
  pub current: ds_core::DiscreteState<'a, ds_core::Bool>,
  pub last_change: ds_core::DiscreteState<'a, ds_core::Float>,
  // Peripherals
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> SquareWaveSource<'a> {
  pub fn builder() -> Builder<'a> {
    Builder {
      __phantom: std::marker::PhantomData,
      val_period: 1e0,
      val_duty: 5e-1,
      val_initial: false,
    }
  }
}

#[allow(non_snake_case)]
pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  val_period: ds_core::Float,
  val_duty: ds_core::Float,
  val_initial: ds_core::Bool,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {
  pub fn period(mut self, v: ds_core::Float) -> Self {
    self.val_period = v;
    self
  }
  pub fn duty(mut self, v: ds_core::Float) -> Self {
    self.val_duty = v;
    self
  }
  pub fn initial(mut self, v: ds_core::Bool) -> Self {
    self.val_initial = v;
    self
  }
}

#[allow(unused_variables)]
impl<'a> ds_core::BlockBuilder<'a, SquareWaveSource<'a>> for Builder<'a> {
  fn build<ST: ds_core::DefaultSystemStrorage>(self, storage_builder: &mut ds_core::SystemStorageBuilder<'a, ST>) -> SquareWaveSource<'a> {
    SquareWaveSource {
      period: storage_builder.create_param(self.val_period),
      duty: storage_builder.create_param(self.val_duty),
      initial: storage_builder.create_param(self.val_initial),

      output: ds_core::Output::new(false),

      current: storage_builder.create_discrete_state(false),
      last_change: storage_builder.create_discrete_state(0e0),

    }
  }
}

impl<'a> ds_core::RequiresStorage for SquareWaveSource<'a> {
  const SIZE: ds_core::StorageSize = ds_core::StorageSize {
    r_param: 2, b_param: 1, i_param: 0,

    r_dstate: 1, b_dstate: 1, i_dstate: 0,
  };
}
