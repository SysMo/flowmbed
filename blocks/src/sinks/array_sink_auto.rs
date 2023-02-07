use flowmbed_dynsys::core as ds_core;
use flowmbed_dynsys::core::DynRefMut;

/// Declare the block struct
#[allow(dead_code)]
pub struct ArraySink<'a, T: Copy + std::fmt::Display, const N: usize> {
  // Inputs
  pub input: ds_core::Input<'a, [T; N]>,
  // Outputs
  // Discrete states
  // Peripherals
  pub sink: DynRefMut<'a, dyn flowmbed_peripherals::sinks::traits::ValueSink>,
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a, T: Copy + std::fmt::Display, const N: usize> ArraySink<'a, T, N> {
  pub fn builder() -> Builder<'a, T, N> {
    Builder {
      __phantom: std::marker::PhantomData,
      __phantom_T: std::marker::PhantomData,
      _sink: None,
    }
  }
}

#[allow(non_snake_case)]
pub struct Builder<'a, T: Copy + std::fmt::Display, const N: usize> {
  __phantom: std::marker::PhantomData<&'a ()>,
  __phantom_T: std::marker::PhantomData<T>,
  _sink: Option<DynRefMut<'a, dyn flowmbed_peripherals::sinks::traits::ValueSink>>,
}

#[allow(dead_code)]
impl<'a, T: Copy + std::fmt::Display, const N: usize> Builder<'a, T, N> {

  pub fn sink(mut self, v: DynRefMut<'a, dyn flowmbed_peripherals::sinks::traits::ValueSink>) -> Self {
    self._sink = Some(v);
    self
  }
}

#[allow(unused_variables)]
impl<'a, T: Copy + std::fmt::Display, const N: usize> ds_core::BlockBuilder<'a, ArraySink<'a, T, N>> for Builder<'a, T, N> {
  fn build<ST: ds_core::DefaultSystemStrorage>(self, storage_builder: &mut ds_core::SystemStorageBuilder<'a, ST>) -> ArraySink<'a, T, N> {
    ArraySink {

      input: ds_core::Input::new(),

      sink: self._sink.unwrap(),

    }
  }
}

impl<'a, T: Copy + std::fmt::Display, const N: usize> ds_core::RequiresStorage for ArraySink<'a, T, N> {
  const SIZE: ds_core::StorageSize = ds_core::StorageSize {
    r_param: 0, b_param: 0, i_param: 0,

    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
