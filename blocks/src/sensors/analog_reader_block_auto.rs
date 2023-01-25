use flowmbed_dynsys::core as dscore;

/// Declare the block struct
#[allow(dead_code)]
pub struct AnalogReaderBlock<'a> {
  // Inputs
  // Outputs
  pub output: dscore::Output<'a, dscore::Float>,
  // Discrete states
  // Peripherals
  pub sensor: &'a mut dyn flowmbed_peripherals::sensors::traits::AnalogReader,
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> AnalogReaderBlock<'a> {
  pub fn builder() -> Builder<'a> {
    Builder {
      __phantom: std::marker::PhantomData,
      periph_sensor: None,
    }
  }
}

pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  periph_sensor: Option<&'a mut dyn flowmbed_peripherals::sensors::traits::AnalogReader>,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {

  pub fn sensor(mut self, v: &'a mut dyn flowmbed_peripherals::sensors::traits::AnalogReader) -> Self {
    self.periph_sensor = Some(v);
    self
  }
}

impl<'a> dscore::BlockBuilder<'a, AnalogReaderBlock<'a>> for Builder<'a> {
  fn build<ST: dscore::DefaultSystemStrorage>(self, storage_builder: &mut dscore::SystemStorageBuilder<'a, ST>) -> AnalogReaderBlock<'a> {
    AnalogReaderBlock {

      output: storage_builder.create_output(0e0),

      sensor: self.periph_sensor.unwrap(),

    }
  }
}

impl<'a> dscore::RequiresStorage for AnalogReaderBlock<'a> {
  const SIZE: dscore::StorageSize = dscore::StorageSize {
    r_param: 0, b_param: 0, i_param: 0,
    r_out: 1, b_out: 0, i_out: 0,
    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
