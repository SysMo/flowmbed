use flowmbed_dynsys::core as ds_core;
use flowmbed_dynsys::core::DynRefMut;

#[allow(unused_imports)]
use flowmbed_dynsys::core::{Float, Int, Bool, String};

/// Declare the block struct
#[allow(dead_code)]
pub struct FloatSink<'a> {
  // Inputs
  pub input: ds_core::Input<'a, ds_core::Float>,
  // Outputs
  // Discrete states
  // Peripherals
  pub sink: DynRefMut<'a, dyn flowmbed_peripherals::sinks::traits::ValueSink>,
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> FloatSink<'a> {
  pub fn builder() -> Builder<'a> {
    Builder {
      __phantom: std::marker::PhantomData,
      _sink: None,
    }
  }
}

#[allow(non_snake_case)]
pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  _sink: Option<DynRefMut<'a, dyn flowmbed_peripherals::sinks::traits::ValueSink>>,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {

  pub fn sink(mut self, v: DynRefMut<'a, dyn flowmbed_peripherals::sinks::traits::ValueSink>) -> Self {
    self._sink = Some(v);
    self
  }
}

#[allow(unused_variables)]
impl<'a> ds_core::BlockBuilder<'a, FloatSink<'a>> for Builder<'a> {
  fn build<ST: ds_core::DefaultSystemStrorage>(self, storage_builder: &mut ds_core::SystemStorageBuilder<'a, ST>) -> FloatSink<'a> {
    FloatSink {

      input: ds_core::Input::new(),

      sink: self._sink.unwrap(),

    }
  }
}

impl<'a> ds_core::RequiresStorage for FloatSink<'a> {
  const SIZE: ds_core::StorageSize = ds_core::StorageSize {
    r_param: 0, b_param: 0, i_param: 0,

    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
