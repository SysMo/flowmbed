use flowmbed_dynsys::core as dscore;

/// Declare the block struct
#[allow(dead_code)]
pub struct SineWaveSource<'a> {
  // Parameters
  pub period: dscore::Parameter<'a, dscore::Float>,
  pub phase: dscore::Parameter<'a, dscore::Float>,
  pub amplitude: dscore::Parameter<'a, dscore::Float>,
  pub offset: dscore::Parameter<'a, dscore::Float>,
  // Inputs
  // Outputs
  pub output: dscore::Output<'a, dscore::Float>,
  // Discrete states
  // Peripherals
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
  val_period: dscore::Float,
  val_phase: dscore::Float,
  val_amplitude: dscore::Float,
  val_offset: dscore::Float,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {
  pub fn period(mut self, v: dscore::Float) -> Self {
    self.val_period = v;
    self
  }
  pub fn phase(mut self, v: dscore::Float) -> Self {
    self.val_phase = v;
    self
  }
  pub fn amplitude(mut self, v: dscore::Float) -> Self {
    self.val_amplitude = v;
    self
  }
  pub fn offset(mut self, v: dscore::Float) -> Self {
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
