use flowmbed_dynsys::core as dscore;

/// Declare the block struct
#[allow(dead_code)]
pub struct SineWaveSource<'a> {
  pub period: dscore::Parameter<'a, f64>,
  pub phase: dscore::Parameter<'a, f64>,
  pub amplitude: dscore::Parameter<'a, f64>,
  pub offset: dscore::Parameter<'a, f64>,

  pub output: dscore::Output<'a, f64>,
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> SineWaveSource<'a> {
  pub fn builder() -> Builder<'a> {
    Builder {
      __phantom: std::marker::PhantomData,
      val_period: 1e0,
      val_phase: 0e0,
      val_amplitude: 1e0,
      val_offset: 0e0,
    }
  }
}

pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  val_period: f64,
  val_phase: f64,
  val_amplitude: f64,
  val_offset: f64,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {
  pub fn period(mut self, v: f64) -> Self {
    self.val_period = v;
    self
  }
  pub fn phase(mut self, v: f64) -> Self {
    self.val_phase = v;
    self
  }
  pub fn amplitude(mut self, v: f64) -> Self {
    self.val_amplitude = v;
    self
  }
  pub fn offset(mut self, v: f64) -> Self {
    self.val_offset = v;
    self
  }
}

impl<'a> dscore::BlockBuilder<'a, SineWaveSource<'a>> for Builder<'a> {
  fn build<ST: dscore::DefaultSystemStrorage>(self, storage_builder: &mut dscore::SystemStorageBuilder<'a, ST>) -> SineWaveSource<'a> {
    SineWaveSource {
      period: storage_builder.create_param(self.val_period),
      phase: storage_builder.create_param(self.val_phase),
      amplitude: storage_builder.create_param(self.val_amplitude),
      offset: storage_builder.create_param(self.val_offset),

      output: storage_builder.create_output(0e0),

    }
  }
}

impl<'a> dscore::RequiresStorage for SineWaveSource<'a> {
  const SIZE: dscore::StorageSize = dscore::StorageSize {
    r_param: 4, b_param: 0, i_param: 0,
    r_out: 1, b_out: 0, i_out: 0,
    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
