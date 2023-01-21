use flowmbed_dynsys::core as dscore;

/// Declare the block struct
#[allow(dead_code)]
pub struct CountingTrigger<'a> {
  // Parameters
  pub pulses_up: dscore::Parameter<'a, dscore::Int>,
  pub pulses_down: dscore::Parameter<'a, dscore::Int>,
  pub initial_state: dscore::Parameter<'a, dscore::Bool>,
  pub initial_count: dscore::Parameter<'a, dscore::Int>,
  pub count_on_rising: dscore::Parameter<'a, dscore::Bool>,
  // Inputs
  pub input: dscore::Input<'a, dscore::Bool>,
  // Outputs
  pub output: dscore::Output<'a, dscore::Bool>,
  // Discrete states
  pub last_input: dscore::DiscreteState<'a, dscore::Bool>,
  pub current: dscore::DiscreteState<'a, dscore::Bool>,
  pub counter: dscore::DiscreteState<'a, dscore::Int>,
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

pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  val_pulses_up: dscore::Int,
  val_pulses_down: dscore::Int,
  val_initial_state: dscore::Bool,
  val_initial_count: dscore::Int,
  val_count_on_rising: dscore::Bool,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {
  pub fn pulses_up(mut self, v: dscore::Int) -> Self {
    self.val_pulses_up = v;
    self
  }
  pub fn pulses_down(mut self, v: dscore::Int) -> Self {
    self.val_pulses_down = v;
    self
  }
  pub fn initial_state(mut self, v: dscore::Bool) -> Self {
    self.val_initial_state = v;
    self
  }
  pub fn initial_count(mut self, v: dscore::Int) -> Self {
    self.val_initial_count = v;
    self
  }
  pub fn count_on_rising(mut self, v: dscore::Bool) -> Self {
    self.val_count_on_rising = v;
    self
  }
}

impl<'a> dscore::BlockBuilder<'a, CountingTrigger<'a>> for Builder<'a> {
  fn build<ST: dscore::DefaultSystemStrorage>(self, storage_builder: &mut dscore::SystemStorageBuilder<'a, ST>) -> CountingTrigger<'a> {
    CountingTrigger {
      pulses_up: storage_builder.create_param(self.val_pulses_up),
      pulses_down: storage_builder.create_param(self.val_pulses_down),
      initial_state: storage_builder.create_param(self.val_initial_state),
      initial_count: storage_builder.create_param(self.val_initial_count),
      count_on_rising: storage_builder.create_param(self.val_count_on_rising),

      input: storage_builder.create_input(),

      output: storage_builder.create_output(false),

      last_input: storage_builder.create_discrete_state(false),
      current: storage_builder.create_discrete_state(false),
      counter: storage_builder.create_discrete_state(0),

    }
  }
}

impl<'a> dscore::RequiresStorage for CountingTrigger<'a> {
  const SIZE: dscore::StorageSize = dscore::StorageSize {
    r_param: 0, b_param: 2, i_param: 3,
    r_out: 0, b_out: 1, i_out: 0,
    r_dstate: 0, b_dstate: 2, i_dstate: 1,
  };
}
