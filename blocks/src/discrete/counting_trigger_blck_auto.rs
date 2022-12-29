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
  pub fn new<ST: dscore::DefaultSystemStrorage>(
    builder: &mut dscore::SystemStorageBuilder<'a, ST>
  ) -> CountingTrigger<'a> {
    CountingTrigger {
      pulses_up: builder.create_param(1),
      pulses_down: builder.create_param(1),
      initial_state: builder.create_param(false),
      initial_count: builder.create_param(0),
      count_on_rising: builder.create_param(true),

      input: builder.create_input(),
      output: builder.create_output(false),

      last_input: builder.create_discrete_state(false),
      current: builder.create_discrete_state(false),
      counter: builder.create_discrete_state(0)
    }
  }

  pub fn builder<ST: dscore::DefaultSystemStrorage>(
    builder: &mut dscore::SystemStorageBuilder<'a, ST>
  ) -> BlockBuilder<'a> {
    BlockBuilder { component: Self::new(builder) }
  }
}

pub struct BlockBuilder<'a> {
  component: CountingTrigger<'a>
}

#[allow(dead_code)]
impl<'a> BlockBuilder<'a> {
  pub fn pulses_up(mut self, v: dscore::Int) -> Self {
    self.component.pulses_up.reset(v);
    self
  }
  pub fn pulses_down(mut self, v: dscore::Int) -> Self {
    self.component.pulses_down.reset(v);
    self
  }
  pub fn initial_state(mut self, v: dscore::Bool) -> Self {
    self.component.initial_state.reset(v);
    self
  }
  pub fn initial_count(mut self, v: dscore::Int) -> Self {
    self.component.initial_count.reset(v);
    self
  }
  pub fn count_on_rising(mut self, v: dscore::Bool) -> Self {
    self.component.count_on_rising.reset(v);
    self
  }

}

impl<'a> From<BlockBuilder<'a>> for CountingTrigger<'a> {
  fn from(x: BlockBuilder<'a>) -> Self {
    x.component
  }
}

impl<'a> dscore::RequiresStorage for CountingTrigger<'a> {
  const SIZE: dscore::StorageSize = dscore::StorageSize {
    r_param: 0, b_param: 2, i_param: 3,
    r_out: 0, b_out: 1, i_out: 0,
    r_dstate: 0, b_dstate: 2, i_dstate: 1,
  };
}
