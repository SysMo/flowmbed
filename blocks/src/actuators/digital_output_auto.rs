use flowmbed_dynsys::core as ds_core;
use flowmbed_dynsys::core::DynRefMut;

#[allow(unused_imports)]
use flowmbed_dynsys::core::{Float, Int, Bool, String};

/// Declare the block struct
#[allow(dead_code)]
pub struct DigitalOutput<'a> {
  // Inputs
  pub input: ds_core::Input<'a, ds_core::Bool>,
  // Outputs
  // Discrete states
  pub current: ds_core::DiscreteState<'a, ds_core::Bool>,
  // Peripherals
  pub periph_out: DynRefMut<'a, dyn flowmbed_peripherals::actuators::traits::DigitalOutputPin>,
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> DigitalOutput<'a> {
  pub fn builder() -> Builder<'a> {
    Builder {
      __phantom: std::marker::PhantomData,
      _periph_out: None,
    }
  }
}

#[allow(non_snake_case)]
pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  _periph_out: Option<DynRefMut<'a, dyn flowmbed_peripherals::actuators::traits::DigitalOutputPin>>,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {

  pub fn periph_out(mut self, v: DynRefMut<'a, dyn flowmbed_peripherals::actuators::traits::DigitalOutputPin>) -> Self {
    self._periph_out = Some(v);
    self
  }
}

#[allow(unused_variables)]
impl<'a> ds_core::BlockBuilder<'a, DigitalOutput<'a>> for Builder<'a> {
  fn build<ST: ds_core::DefaultSystemStrorage>(self, storage_builder: &mut ds_core::SystemStorageBuilder<'a, ST>) -> DigitalOutput<'a> {
    DigitalOutput {

      input: ds_core::Input::new(),

      current: storage_builder.create_discrete_state(false),

      periph_out: self._periph_out.unwrap(),

    }
  }
}

impl<'a> ds_core::RequiresStorage for DigitalOutput<'a> {
  const SIZE: ds_core::StorageSize = ds_core::StorageSize {
    r_param: 0, b_param: 0, i_param: 0,

    r_dstate: 0, b_dstate: 1, i_dstate: 0,
  };
}
