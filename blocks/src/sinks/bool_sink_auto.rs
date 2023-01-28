use flowmbed_dynsys::core as dscore;
use flowmbed_dynsys::core::DynRefMut;

/// Declare the block struct
#[allow(dead_code)]
pub struct BoolSink<'a> {
  // Inputs
  pub input: dscore::Input<'a, dscore::Bool>,
  // Outputs
  // Discrete states
  // Peripherals
  pub sink: DynRefMut<'a, dyn flowmbed_peripherals::sinks::traits::ValueSink>,
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> BoolSink<'a> {
  pub fn builder() -> Builder<'a> {
    Builder {
      __phantom: std::marker::PhantomData,
      periph_sink: None,
    }
  }
}

pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  periph_sink: Option<DynRefMut<'a, dyn flowmbed_peripherals::sinks::traits::ValueSink>>,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {

  pub fn sink(mut self, v: DynRefMut<'a, dyn flowmbed_peripherals::sinks::traits::ValueSink>) -> Self {
    self.periph_sink = Some(v);
    self
  }
}

impl<'a> dscore::BlockBuilder<'a, BoolSink<'a>> for Builder<'a> {
  fn build<ST: dscore::DefaultSystemStrorage>(self, storage_builder: &mut dscore::SystemStorageBuilder<'a, ST>) -> BoolSink<'a> {
    BoolSink {

      input: storage_builder.create_input(),

      sink: self.periph_sink.unwrap(),

    }
  }
}

impl<'a> dscore::RequiresStorage for BoolSink<'a> {
  const SIZE: dscore::StorageSize = dscore::StorageSize {
    r_param: 0, b_param: 0, i_param: 0,
    r_out: 0, b_out: 0, i_out: 0,
    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
