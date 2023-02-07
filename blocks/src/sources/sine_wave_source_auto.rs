use flowmbed_dynsys::core as ds_core;

/// Declare the block struct
#[allow(dead_code)]
pub struct SineWaveSource<'a> {
  // Parameters
  pub period: ds_core::Parameter<'a, ds_core::Float>,
  pub phase: ds_core::Parameter<'a, ds_core::Float>,
  pub amplitude: ds_core::Parameter<'a, ds_core::Float>,
  pub offset: ds_core::Parameter<'a, ds_core::Float>,
  // Inputs
  // Outputs
  pub output: ds_core::Output<ds_core::Float>,
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

#[allow(non_snake_case)]
pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  val_period: ds_core::Float,
  val_phase: ds_core::Float,
  val_amplitude: ds_core::Float,
  val_offset: ds_core::Float,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {
  pub fn period(mut self, v: ds_core::Float) -> Self {
    self.val_period = v;
    self
  }
  pub fn phase(mut self, v: ds_core::Float) -> Self {
    self.val_phase = v;
    self
  }
  pub fn amplitude(mut self, v: ds_core::Float) -> Self {
    self.val_amplitude = v;
    self
  }
  pub fn offset(mut self, v: ds_core::Float) -> Self {
    self.val_offset = v;
    self
  }
}

#[allow(unused_variables)]
impl<'a> ds_core::BlockBuilder<'a, SineWaveSource<'a>> for Builder<'a> {
  fn build<ST: ds_core::DefaultSystemStrorage>(self, storage_builder: &mut ds_core::SystemStorageBuilder<'a, ST>) -> SineWaveSource<'a> {
    SineWaveSource {
      period: storage_builder.create_param(self.val_period),
      phase: storage_builder.create_param(self.val_phase),
      amplitude: storage_builder.create_param(self.val_amplitude),
      offset: storage_builder.create_param(self.val_offset),

      output: ds_core::Output::new(0e0),

    }
  }
}

impl<'a> ds_core::RequiresStorage for SineWaveSource<'a> {
  const SIZE: ds_core::StorageSize = ds_core::StorageSize {
    r_param: 4, b_param: 0, i_param: 0,

    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
