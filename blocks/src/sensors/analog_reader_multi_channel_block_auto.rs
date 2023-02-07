use flowmbed_dynsys::core as ds_core;
use flowmbed_dynsys::core::DynRefMut;

/// Declare the block struct
#[allow(dead_code)]
pub struct AnalogReaderMultiChannelBlock<'a, const N: usize> {
  // Inputs
  // Outputs
  pub readings: ds_core::Output<[ds_core::Float; N]>,
  // Discrete states
  // Peripherals
  pub periph_reader: DynRefMut<'a, dyn flowmbed_peripherals::sensors::traits::AnalogReaderMultiChannel<N>>,
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a, const N: usize> AnalogReaderMultiChannelBlock<'a, N> {
  pub fn builder() -> Builder<'a, N> {
    Builder {
      __phantom: std::marker::PhantomData,
      _periph_reader: None,
    }
  }
}

#[allow(non_snake_case)]
pub struct Builder<'a, const N: usize> {
  __phantom: std::marker::PhantomData<&'a ()>,
  _periph_reader: Option<DynRefMut<'a, dyn flowmbed_peripherals::sensors::traits::AnalogReaderMultiChannel<N>>>,
}

#[allow(dead_code)]
impl<'a, const N: usize> Builder<'a, N> {

  pub fn periph_reader(mut self, v: DynRefMut<'a, dyn flowmbed_peripherals::sensors::traits::AnalogReaderMultiChannel<N>>) -> Self {
    self._periph_reader = Some(v);
    self
  }
}

#[allow(unused_variables)]
impl<'a, const N: usize> ds_core::BlockBuilder<'a, AnalogReaderMultiChannelBlock<'a, N>> for Builder<'a, N> {
  fn build<ST: ds_core::DefaultSystemStrorage>(self, storage_builder: &mut ds_core::SystemStorageBuilder<'a, ST>) -> AnalogReaderMultiChannelBlock<'a, N> {
    AnalogReaderMultiChannelBlock {

      readings: ds_core::Output::new(0e0),

      periph_reader: self._periph_reader.unwrap(),

    }
  }
}

impl<'a, const N: usize> ds_core::RequiresStorage for AnalogReaderMultiChannelBlock<'a, N> {
  const SIZE: ds_core::StorageSize = ds_core::StorageSize {
    r_param: 0, b_param: 0, i_param: 0,

    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
