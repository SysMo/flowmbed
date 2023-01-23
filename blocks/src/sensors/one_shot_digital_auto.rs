use flowmbed_dynsys::core as dscore;

/// Declare the block struct
#[allow(dead_code)]
pub struct OneShotDigital<'a> {
  // Inputs
  // Outputs
  pub output: dscore::Output<'a, dscore::Bool>,
  // Discrete states
  // Peripherals
  pub sensor: &'a mut dyn flowmbed_peripherals::sensors::traits::OneShotDigital,
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> OneShotDigital<'a> {
  pub fn builder() -> Builder<'a> {
    Builder {
      __phantom: std::marker::PhantomData,
      periph_sensor: None,
    }
  }
}

pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  periph_sensor: Option<&'a mut dyn flowmbed_peripherals::sensors::traits::OneShotDigital>,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {

  pub fn sensor(mut self, v: &'a mut dyn flowmbed_peripherals::sensors::traits::OneShotDigital) -> Self {
    self.periph_sensor = Some(v);
    self
  }
}

impl<'a> dscore::BlockBuilder<'a, OneShotDigital<'a>> for Builder<'a> {
  fn build<ST: dscore::DefaultSystemStrorage>(self, storage_builder: &mut dscore::SystemStorageBuilder<'a, ST>) -> OneShotDigital<'a> {
    OneShotDigital {

      output: storage_builder.create_output(false),

      sensor: self.periph_sensor.unwrap(),

    }
  }
}

impl<'a> dscore::RequiresStorage for OneShotDigital<'a> {
  const SIZE: dscore::StorageSize = dscore::StorageSize {
    r_param: 0, b_param: 0, i_param: 0,
    r_out: 0, b_out: 1, i_out: 0,
    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
