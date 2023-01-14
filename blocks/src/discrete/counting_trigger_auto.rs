use flowmbed_dynsys::core as dscore;

/// Declare the block struct
#[allow(dead_code)]
pub struct CountingTrigger<'a> {
  pub pulses_up: dscore::Parameter<'a, i64>,
  pub pulses_down: dscore::Parameter<'a, i64>,
  pub initial_state: dscore::Parameter<'a, bool>,
  pub initial_count: dscore::Parameter<'a, i64>,
  pub count_on_rising: dscore::Parameter<'a, bool>,

  pub input: dscore::Input<'a, bool>,

  pub output: dscore::Output<'a, bool>,

  pub last_input: dscore::DiscreteState<'a, bool>,
  pub current: dscore::DiscreteState<'a, bool>,
  pub counter: dscore::DiscreteState<'a, i64>,
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
  val_pulses_up: i64,
  val_pulses_down: i64,
  val_initial_state: bool,
  val_initial_count: i64,
  val_count_on_rising: bool,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {
  pub fn pulses_up(mut self, v: i64) -> Self {
    self.val_pulses_up = v;
    self
  }
  pub fn pulses_down(mut self, v: i64) -> Self {
    self.val_pulses_down = v;
    self
  }
  pub fn initial_state(mut self, v: bool) -> Self {
    self.val_initial_state = v;
    self
  }
  pub fn initial_count(mut self, v: i64) -> Self {
    self.val_initial_count = v;
    self
  }
  pub fn count_on_rising(mut self, v: bool) -> Self {
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
