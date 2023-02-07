use flowmbed_dynsys::core as ds_core;
use flowmbed_dynsys::core::DynRefMut;

#[allow(unused_imports)]
use flowmbed_dynsys::core::{Float, Int, Bool, String};

/// Declare the block struct
#[allow(dead_code)]
pub struct PwmMultiChannelBlock<'a, const N: usize> {
  // Inputs
  pub duty: ds_core::Input<'a, [ds_core::Float; N]>,
  // Outputs
  // Discrete states
  // Peripherals
  pub periph_out: DynRefMut<'a, dyn flowmbed_peripherals::actuators::traits::PwmMultiChannel<N>>,
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a, const N: usize> PwmMultiChannelBlock<'a, N> {
  pub fn builder() -> Builder<'a, N> {
    Builder {
      __phantom: std::marker::PhantomData,
      _periph_out: None,
    }
  }
}

#[allow(non_snake_case)]
pub struct Builder<'a, const N: usize> {
  __phantom: std::marker::PhantomData<&'a ()>,
  _periph_out: Option<DynRefMut<'a, dyn flowmbed_peripherals::actuators::traits::PwmMultiChannel<N>>>,
}

#[allow(dead_code)]
impl<'a, const N: usize> Builder<'a, N> {

  pub fn periph_out(mut self, v: DynRefMut<'a, dyn flowmbed_peripherals::actuators::traits::PwmMultiChannel<N>>) -> Self {
    self._periph_out = Some(v);
    self
  }
}

#[allow(unused_variables)]
impl<'a, const N: usize> ds_core::BlockBuilder<'a, PwmMultiChannelBlock<'a, N>> for Builder<'a, N> {
  fn build<ST: ds_core::DefaultSystemStrorage>(self, storage_builder: &mut ds_core::SystemStorageBuilder<'a, ST>) -> PwmMultiChannelBlock<'a, N> {
    PwmMultiChannelBlock {

      duty: ds_core::Input::new(),

      periph_out: self._periph_out.unwrap(),

    }
  }
}

impl<'a, const N: usize> ds_core::RequiresStorage for PwmMultiChannelBlock<'a, N> {
  const SIZE: ds_core::StorageSize = ds_core::StorageSize {
    r_param: 0, b_param: 0, i_param: 0,

    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
