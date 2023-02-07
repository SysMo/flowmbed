use flowmbed_dynsys::core as ds_core;
use flowmbed_dynsys::core::DynRefMut;

#[allow(unused_imports)]
use flowmbed_dynsys::core::{Float, Int, Bool, String};

/// Declare the block struct
#[allow(dead_code)]
pub struct DigitalReaderBlock<'a> {
  // Inputs
  // Outputs
  pub output: ds_core::Output<ds_core::Bool>,
  // Discrete states
  // Peripherals
  pub periph_reader: DynRefMut<'a, dyn flowmbed_peripherals::sensors::traits::DigitalReader>,
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> DigitalReaderBlock<'a> {
  pub fn builder() -> Builder<'a> {
    Builder {
      __phantom: std::marker::PhantomData,
      _periph_reader: None,
    }
  }
}

#[allow(non_snake_case)]
pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  _periph_reader: Option<DynRefMut<'a, dyn flowmbed_peripherals::sensors::traits::DigitalReader>>,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {

  pub fn periph_reader(mut self, v: DynRefMut<'a, dyn flowmbed_peripherals::sensors::traits::DigitalReader>) -> Self {
    self._periph_reader = Some(v);
    self
  }
}

#[allow(unused_variables)]
impl<'a> ds_core::BlockBuilder<'a, DigitalReaderBlock<'a>> for Builder<'a> {
  fn build<ST: ds_core::DefaultSystemStrorage>(self, storage_builder: &mut ds_core::SystemStorageBuilder<'a, ST>) -> DigitalReaderBlock<'a> {
    DigitalReaderBlock {

      output: ds_core::Output::new(false),

      periph_reader: self._periph_reader.unwrap(),

    }
  }
}

impl<'a> ds_core::RequiresStorage for DigitalReaderBlock<'a> {
  const SIZE: ds_core::StorageSize = ds_core::StorageSize {
    r_param: 0, b_param: 0, i_param: 0,

    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
