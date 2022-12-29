use flowmbed_dynsys::core as dscore;

#[doc=" Declare the block struct"]
#[allow(dead_code)]
pub struct SquareWaveSource<'a> {
  pub period: dscore::Parameter<'a, dscore::Float>,
  pub duty: dscore::Parameter<'a, dscore::Float>,
  pub initial: dscore::Parameter<'a, dscore::Bool>,

  pub output: dscore::Output<'a, dscore::Bool>,

  pub current: dscore::DiscreteState<'a, dscore::Bool>,
  pub last_change: dscore::DiscreteState<'a, dscore::Float>,
}

#[doc=" Implement the block struct"]
#[allow(dead_code)]
impl<'a> SquareWaveSource<'a> {
  pub fn new<ST: dscore::DefaultSystemStrorage>(
    builder: &mut dscore::SystemStorageBuilder<'a, ST>
  ) -> SquareWaveSource<'a> {
    SquareWaveSource {
      period: builder.create_param(1e0),
      duty: builder.create_param(5e-1),
      initial: builder.create_param(false),

      output: builder.create_output(false),

      current: builder.create_discrete_state(false),
      last_change: builder.create_discrete_state(0e0)
    }
  }

  pub fn builder<ST: dscore::DefaultSystemStrorage>(
    builder: &mut dscore::SystemStorageBuilder<'a, ST>
  ) -> BlockBuilder<'a> {
    BlockBuilder { component: Self::new(builder) }
  }
}

pub struct BlockBuilder<'a> {
  component: SquareWaveSource<'a>
}

#[allow(dead_code)]
impl<'a> BlockBuilder<'a> {
  pub fn period(mut self, v: dscore::Float) -> Self {
    self.component.period.reset(v);
    self
  }
  pub fn duty(mut self, v: dscore::Float) -> Self {
    self.component.duty.reset(v);
    self
  }
  pub fn initial(mut self, v: dscore::Bool) -> Self {
    self.component.initial.reset(v);
    self
  }

}

impl<'a> From<BlockBuilder<'a>> for SquareWaveSource<'a> {
  fn from(x: BlockBuilder<'a>) -> Self {
    x.component
  }
}

impl<'a> dscore::RequiresStorage for SquareWaveSource<'a> {
  const SIZE: dscore::StorageSize = dscore::StorageSize {
    r_param: 2, b_param: 1, i_param: 0,
    r_out: 0, b_out: 1, i_out: 0,
    r_dstate: 1, b_dstate: 1, i_dstate: 0,
  };
}
