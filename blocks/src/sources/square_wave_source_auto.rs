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

  pub fn builder<ST: dscore::DefaultSystemStrorage>(
    storage_builder: &'a mut dscore::SystemStorageBuilder<'a, ST>
  ) -> BlockBuilder<'a, ST> {
    BlockBuilder {
      __storage_builder: storage_builder,
      val_period: 1e0,
      val_duty: 5e-1,
      val_initial: false,
    }
  }
}

pub struct BlockBuilder<'a, ST: dscore::DefaultSystemStrorage> {
  __storage_builder: &'a mut dscore::SystemStorageBuilder<'a, ST>,
  val_period: dscore::Float,
  val_duty: dscore::Float,
  val_initial: dscore::Bool,
}

#[allow(dead_code)]
impl<'a, ST: dscore::DefaultSystemStrorage> BlockBuilder<'a, ST> {
  pub fn period(mut self, v: dscore::Float) -> Self {
    self.val_period = v;
    self
  }
  pub fn duty(mut self, v: dscore::Float) -> Self {
    self.val_duty = v;
    self
  }
  pub fn initial(mut self, v: dscore::Bool) -> Self {
    self.val_initial = v;
    self
  }
}

impl<'a, ST: dscore::DefaultSystemStrorage> From<BlockBuilder<'a, ST>> for SquareWaveSource<'a> {
  fn from(builder: BlockBuilder<'a, ST>) -> Self {
    SquareWaveSource {
      period: builder.__storage_builder.create_param(builder.val_period),
      duty: builder.__storage_builder.create_param(builder.val_duty),
      initial: builder.__storage_builder.create_param(builder.val_initial),

      output: builder.__storage_builder.create_output(false),

      current: builder.__storage_builder.create_discrete_state(false),
      last_change: builder.__storage_builder.create_discrete_state(0e0),

    }
  }
}

impl<'a> dscore::RequiresStorage for SquareWaveSource<'a> {
  const SIZE: dscore::StorageSize = dscore::StorageSize {
    r_param: 2, b_param: 1, i_param: 0,
    r_out: 0, b_out: 1, i_out: 0,
    r_dstate: 1, b_dstate: 1, i_dstate: 0,
  };
}
