use flowmbed_dynsys::core as dscore;

/// Declare the block struct
#[allow(dead_code)]
pub struct SquareWaveSource<'a> {
  pub period: dscore::Parameter<'a, f64>,
  pub duty: dscore::Parameter<'a, f64>,
  pub initial: dscore::Parameter<'a, bool>,

  pub output: dscore::Output<'a, bool>,

  pub current: dscore::DiscreteState<'a, bool>,
  pub last_change: dscore::DiscreteState<'a, f64>,
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

pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a i32>,
  val_period: f64,
  val_duty: f64,
  val_initial: bool,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {
  pub fn period(mut self, v: f64) -> Self {
    self.val_period = v;
    self
  }
  pub fn duty(mut self, v: f64) -> Self {
    self.val_duty = v;
    self
  }
  pub fn initial(mut self, v: bool) -> Self {
    self.val_initial = v;
    self
  }
}

impl<'a> dscore::BlockBuilder<'a, SquareWaveSource<'a>> for Builder<'a> {
  fn build<ST: dscore::DefaultSystemStrorage>(self, storage_builder: &mut dscore::SystemStorageBuilder<'a, ST>) -> SquareWaveSource<'a> {
    SquareWaveSource {
      period: storage_builder.create_param(self.val_period),
      duty: storage_builder.create_param(self.val_duty),
      initial: storage_builder.create_param(self.val_initial),

      output: storage_builder.create_output(false),

      current: storage_builder.create_discrete_state(false),
      last_change: storage_builder.create_discrete_state(0e0),

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
