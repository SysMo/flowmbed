use flowmbed_dynsys::core as dscore;
use flowmbed_dynsys::core::DynRefMut;

/// Declare the block struct
#[allow(dead_code)]
pub struct AnalogReaderMultiChannelBlock<'a> {
  // Inputs
  // Outputs
  pub output1: dscore::Output<'a, dscore::Float>,
  pub output2: dscore::Output<'a, dscore::Float>,
  // Discrete states
  // Peripherals
  pub sensor: DynRefMut<'a, dyn flowmbed_peripherals::sensors::traits::AnalogReaderMultiChannel<2>>,
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> AnalogReaderMultiChannelBlock<'a> {
  pub fn builder() -> Builder<'a> {
    Builder {
      __phantom: std::marker::PhantomData,
      periph_sensor: None,
    }
  }
}

pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  periph_sensor: Option<DynRefMut<'a, dyn flowmbed_peripherals::sensors::traits::AnalogReaderMultiChannel<2>>>,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {

  pub fn sensor(mut self, v: DynRefMut<'a, dyn flowmbed_peripherals::sensors::traits::AnalogReaderMultiChannel<2>>) -> Self {
    self.periph_sensor = Some(v);
    self
  }
}

impl<'a> dscore::BlockBuilder<'a, AnalogReaderMultiChannelBlock<'a>> for Builder<'a> {
  fn build<ST: dscore::DefaultSystemStrorage>(self, storage_builder: &mut dscore::SystemStorageBuilder<'a, ST>) -> AnalogReaderMultiChannelBlock<'a> {
    AnalogReaderMultiChannelBlock {

      output1: storage_builder.create_output(0e0),
      output2: storage_builder.create_output(0e0),

      sensor: self.periph_sensor.unwrap(),

    }
  }
}

impl<'a> dscore::RequiresStorage for AnalogReaderMultiChannelBlock<'a> {
  const SIZE: dscore::StorageSize = dscore::StorageSize {
    r_param: 0, b_param: 0, i_param: 0,
    r_out: 2, b_out: 0, i_out: 0,
    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
