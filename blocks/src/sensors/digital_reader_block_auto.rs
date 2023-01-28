use flowmbed_dynsys::core as dscore;
use flowmbed_dynsys::core::DynRefMut;

/// Declare the block struct
#[allow(dead_code)]
pub struct DigitalReaderBlock<'a> {
  // Inputs
  // Outputs
  pub output: dscore::Output<'a, dscore::Bool>,
  // Discrete states
  // Peripherals
  pub sensor: DynRefMut<'a, dyn flowmbed_peripherals::sensors::traits::DigitalReader>,
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> DigitalReaderBlock<'a> {
  pub fn builder() -> Builder<'a> {
    Builder {
      __phantom: std::marker::PhantomData,
      periph_sensor: None,
    }
  }
}

pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  periph_sensor: Option<DynRefMut<'a, dyn flowmbed_peripherals::sensors::traits::DigitalReader>>,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {

  pub fn sensor(mut self, v: DynRefMut<'a, dyn flowmbed_peripherals::sensors::traits::DigitalReader>) -> Self {
    self.periph_sensor = Some(v);
    self
  }
}

impl<'a> dscore::BlockBuilder<'a, DigitalReaderBlock<'a>> for Builder<'a> {
  fn build<ST: dscore::DefaultSystemStrorage>(self, storage_builder: &mut dscore::SystemStorageBuilder<'a, ST>) -> DigitalReaderBlock<'a> {
    DigitalReaderBlock {

      output: storage_builder.create_output(false),

      sensor: self.periph_sensor.unwrap(),

    }
  }
}

impl<'a> dscore::RequiresStorage for DigitalReaderBlock<'a> {
  const SIZE: dscore::StorageSize = dscore::StorageSize {
    r_param: 0, b_param: 0, i_param: 0,
    r_out: 0, b_out: 1, i_out: 0,
    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
