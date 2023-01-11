use flowmbed_dynsys::core as dscore;

#[doc=" Declare the block struct"]
#[allow(dead_code)]
pub struct SineWaveSource<'a> {
  pub period: dscore::Parameter<'a, dscore::Float>,
  pub phase: dscore::Parameter<'a, dscore::Float>,
  pub amplitude: dscore::Parameter<'a, dscore::Float>,

  pub output: dscore::Output<'a, dscore::Float>,
}

#[doc=" Implement the block struct"]
#[allow(dead_code)]
impl<'a> SineWaveSource<'a> {

  pub fn builder<ST: dscore::DefaultSystemStrorage>(
    storage_builder: &'a mut dscore::SystemStorageBuilder<'a, ST>
  ) -> BlockBuilder<'a, ST> {
    BlockBuilder {
      __storage_builder: storage_builder,
      val_period: 1e0,
      val_phase: 0e0,
      val_amplitude: 1e0,
    }
  }
}

pub struct BlockBuilder<'a, ST: dscore::DefaultSystemStrorage> {
  __storage_builder: &'a mut dscore::SystemStorageBuilder<'a, ST>,
  val_period: dscore::Float,
  val_phase: dscore::Float,
  val_amplitude: dscore::Float,
}

#[allow(dead_code)]
impl<'a, ST: dscore::DefaultSystemStrorage> BlockBuilder<'a, ST> {
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
}

impl<'a, ST: dscore::DefaultSystemStrorage> From<BlockBuilder<'a, ST>> for SineWaveSource<'a> {
  fn from(builder: BlockBuilder<'a, ST>) -> Self {
    SineWaveSource {
      period: builder.__storage_builder.create_param(builder.val_period),
      phase: builder.__storage_builder.create_param(builder.val_phase),
      amplitude: builder.__storage_builder.create_param(builder.val_amplitude),

      output: builder.__storage_builder.create_output(0e0),

    }
  }
}

impl<'a> dscore::RequiresStorage for SineWaveSource<'a> {
  const SIZE: dscore::StorageSize = dscore::StorageSize {
    r_param: 3, b_param: 0, i_param: 0,
    r_out: 1, b_out: 0, i_out: 0,
    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
