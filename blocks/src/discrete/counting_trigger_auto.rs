use flowmbed_dynsys::core as dscore;

#[doc=" Declare the block struct"]
#[allow(dead_code)]
pub struct CountingTrigger<'a> {
  pub pulses_up: dscore::Parameter<'a, dscore::Int>,
  pub pulses_down: dscore::Parameter<'a, dscore::Int>,
  pub initial_state: dscore::Parameter<'a, dscore::Bool>,
  pub initial_count: dscore::Parameter<'a, dscore::Int>,
  pub count_on_rising: dscore::Parameter<'a, dscore::Bool>,

  pub input: dscore::Input<'a, dscore::Bool>,

  pub output: dscore::Output<'a, dscore::Bool>,

  pub last_input: dscore::DiscreteState<'a, dscore::Bool>,
  pub current: dscore::DiscreteState<'a, dscore::Bool>,
  pub counter: dscore::DiscreteState<'a, dscore::Int>,
}

#[doc=" Implement the block struct"]
#[allow(dead_code)]
impl<'a> CountingTrigger<'a> {

  pub fn builder<ST: dscore::DefaultSystemStrorage>(
    storage_builder: &'a mut dscore::SystemStorageBuilder<'a, ST>
  ) -> BlockBuilder<'a, ST> {
    BlockBuilder {
      __storage_builder: storage_builder,
      val_pulses_up: 1,
      val_pulses_down: 1,
      val_initial_state: false,
      val_initial_count: 0,
      val_count_on_rising: true,
    }
  }
}

pub struct BlockBuilder<'a, ST: dscore::DefaultSystemStrorage> {
  __storage_builder: &'a mut dscore::SystemStorageBuilder<'a, ST>,
  val_pulses_up: dscore::Int,
  val_pulses_down: dscore::Int,
  val_initial_state: dscore::Bool,
  val_initial_count: dscore::Int,
  val_count_on_rising: dscore::Bool,
}

#[allow(dead_code)]
impl<'a, ST: dscore::DefaultSystemStrorage> BlockBuilder<'a, ST> {
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

impl<'a, ST: dscore::DefaultSystemStrorage> From<BlockBuilder<'a, ST>> for CountingTrigger<'a> {
  fn from(builder: BlockBuilder<'a, ST>) -> Self {
    CountingTrigger {
      pulses_up: builder.__storage_builder.create_param(builder.val_pulses_up),
      pulses_down: builder.__storage_builder.create_param(builder.val_pulses_down),
      initial_state: builder.__storage_builder.create_param(builder.val_initial_state),
      initial_count: builder.__storage_builder.create_param(builder.val_initial_count),
      count_on_rising: builder.__storage_builder.create_param(builder.val_count_on_rising),

      input: builder.__storage_builder.create_input(),

      output: builder.__storage_builder.create_output(false),

      last_input: builder.__storage_builder.create_discrete_state(false),
      current: builder.__storage_builder.create_discrete_state(false),
      counter: builder.__storage_builder.create_discrete_state(0),

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
