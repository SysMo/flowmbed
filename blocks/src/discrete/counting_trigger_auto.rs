use flowmbed_dynsys::core as ds_core;

#[allow(unused_imports)]
use flowmbed_dynsys::core::{Float, Int, Bool, String};

/// Declare the block struct
#[allow(dead_code)]
pub struct CountingTrigger<'a> {
  // Parameters
  pub pulses_up: ds_core::Parameter<'a, ds_core::Int>,
  pub pulses_down: ds_core::Parameter<'a, ds_core::Int>,
  pub initial_state: ds_core::Parameter<'a, ds_core::Bool>,
  pub initial_count: ds_core::Parameter<'a, ds_core::Int>,
  pub count_on_rising: ds_core::Parameter<'a, ds_core::Bool>,
  // Inputs
  pub input: ds_core::Input<'a, ds_core::Bool>,
  // Outputs
  pub output: ds_core::Output<ds_core::Bool>,
  // Discrete states
  pub last_input: ds_core::DiscreteState<'a, ds_core::Bool>,
  pub current: ds_core::DiscreteState<'a, ds_core::Bool>,
  pub counter: ds_core::DiscreteState<'a, ds_core::Int>,
  // Peripherals
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> CountingTrigger<'a> {
  pub fn builder() -> Builder<'a> {
    Builder {
      __phantom: std::marker::PhantomData,
      val_pulses_up: 1,
      val_pulses_down: 1,
      val_initial_state: false,
      val_initial_count: 0,
      val_count_on_rising: true,
    }
  }
}

#[allow(non_snake_case)]
pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  val_pulses_up: ds_core::Int,
  val_pulses_down: ds_core::Int,
  val_initial_state: ds_core::Bool,
  val_initial_count: ds_core::Int,
  val_count_on_rising: ds_core::Bool,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {
  pub fn pulses_up(mut self, v: ds_core::Int) -> Self {
    self.val_pulses_up = v;
    self
  }
  pub fn pulses_down(mut self, v: ds_core::Int) -> Self {
    self.val_pulses_down = v;
    self
  }
  pub fn initial_state(mut self, v: ds_core::Bool) -> Self {
    self.val_initial_state = v;
    self
  }
  pub fn initial_count(mut self, v: ds_core::Int) -> Self {
    self.val_initial_count = v;
    self
  }
  pub fn count_on_rising(mut self, v: ds_core::Bool) -> Self {
    self.val_count_on_rising = v;
    self
  }
}

#[allow(unused_variables)]
impl<'a> ds_core::BlockBuilder<'a, CountingTrigger<'a>> for Builder<'a> {
  fn build<ST: ds_core::DefaultSystemStrorage>(self, storage_builder: &mut ds_core::SystemStorageBuilder<'a, ST>) -> CountingTrigger<'a> {
    CountingTrigger {
      pulses_up: storage_builder.create_param(self.val_pulses_up),
      pulses_down: storage_builder.create_param(self.val_pulses_down),
      initial_state: storage_builder.create_param(self.val_initial_state),
      initial_count: storage_builder.create_param(self.val_initial_count),
      count_on_rising: storage_builder.create_param(self.val_count_on_rising),

      input: ds_core::Input::new(),

      output: ds_core::Output::new(false),

      last_input: storage_builder.create_discrete_state(false),
      current: storage_builder.create_discrete_state(false),
      counter: storage_builder.create_discrete_state(0),

    }
  }
}

impl<'a> ds_core::RequiresStorage for CountingTrigger<'a> {
  const SIZE: ds_core::StorageSize = ds_core::StorageSize {
    r_param: 0, b_param: 2, i_param: 3,

    r_dstate: 0, b_dstate: 2, i_dstate: 1,
  };
}
