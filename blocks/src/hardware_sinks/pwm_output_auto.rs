use flowmbed_dynsys::core as dscore;

/// Declare the block struct
#[allow(dead_code)]
pub struct PWMOutput<'a> {
  // Inputs
  pub duty: dscore::Input<'a, dscore::Float>,
  // Outputs
  // Discrete states
  // Peripherals
  pub out: &'a mut dyn flowmbed_peripherals::actuators::traits::PwmPin,
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> PWMOutput<'a> {
  pub fn builder() -> Builder<'a> {
    Builder {
      __phantom: std::marker::PhantomData,
      periph_out: None,
    }
  }
}

pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  periph_out: Option<&'a mut dyn flowmbed_peripherals::actuators::traits::PwmPin>,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {

  pub fn out(mut self, v: &'a mut dyn flowmbed_peripherals::actuators::traits::PwmPin) -> Self {
    self.periph_out = Some(v);
    self
  }
}

impl<'a> dscore::BlockBuilder<'a, PWMOutput<'a>> for Builder<'a> {
  fn build<ST: dscore::DefaultSystemStrorage>(self, storage_builder: &mut dscore::SystemStorageBuilder<'a, ST>) -> PWMOutput<'a> {
    PWMOutput {

      duty: storage_builder.create_input(),

      out: self.periph_out.unwrap(),

    }
  }
}

impl<'a> dscore::RequiresStorage for PWMOutput<'a> {
  const SIZE: dscore::StorageSize = dscore::StorageSize {
    r_param: 0, b_param: 0, i_param: 0,
    r_out: 0, b_out: 0, i_out: 0,
    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
