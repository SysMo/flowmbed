use flowmbed_dynsys::core as ds_core;
use flowmbed_dynsys::core::DynRefMut;

/// Declare the block struct
#[allow(dead_code)]
pub struct PWMOutput<'a> {
  // Inputs
  pub duty: ds_core::Input<'a, ds_core::Float>,
  // Outputs
  // Discrete states
  // Peripherals
  pub periph_out: DynRefMut<'a, dyn flowmbed_peripherals::actuators::traits::PwmChannel>,
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> PWMOutput<'a> {
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
  _periph_out: Option<DynRefMut<'a, dyn flowmbed_peripherals::actuators::traits::PwmChannel>>,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {

  pub fn periph_out(mut self, v: DynRefMut<'a, dyn flowmbed_peripherals::actuators::traits::PwmChannel>) -> Self {
    self._periph_out = Some(v);
    self
  }
}

#[allow(unused_variables)]
impl<'a> ds_core::BlockBuilder<'a, PWMOutput<'a>> for Builder<'a> {
  fn build<ST: ds_core::DefaultSystemStrorage>(self, storage_builder: &mut ds_core::SystemStorageBuilder<'a, ST>) -> PWMOutput<'a> {
    PWMOutput {

      duty: ds_core::Input::new(),

      periph_out: self._periph_out.unwrap(),

    }
  }
}

impl<'a> ds_core::RequiresStorage for PWMOutput<'a> {
  const SIZE: ds_core::StorageSize = ds_core::StorageSize {
    r_param: 0, b_param: 0, i_param: 0,

    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
