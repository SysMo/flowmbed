use flowmbed_dynsys::core as dscore;
use flowmbed_dynsys::core::DynRefMut;

/// Declare the block struct
#[allow(dead_code)]
pub struct DigitalOutput<'a> {
  // Inputs
  pub input: dscore::Input<'a, dscore::Bool>,
  // Outputs
  // Discrete states
  pub current: dscore::DiscreteState<'a, dscore::Bool>,
  // Peripherals
  pub output: DynRefMut<'a, dyn flowmbed_peripherals::actuators::traits::DigitalOutputPin>,
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> DigitalOutput<'a> {
  pub fn builder() -> Builder<'a> {
    Builder {
      __phantom: std::marker::PhantomData,
      periph_output: None,
    }
  }
}

pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  periph_output: Option<DynRefMut<'a, dyn flowmbed_peripherals::actuators::traits::DigitalOutputPin>>,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {

  pub fn output(mut self, v: DynRefMut<'a, dyn flowmbed_peripherals::actuators::traits::DigitalOutputPin>) -> Self {
    self.periph_output = Some(v);
    self
  }
}

impl<'a> dscore::BlockBuilder<'a, DigitalOutput<'a>> for Builder<'a> {
  fn build<ST: dscore::DefaultSystemStrorage>(self, storage_builder: &mut dscore::SystemStorageBuilder<'a, ST>) -> DigitalOutput<'a> {
    DigitalOutput {

      input: storage_builder.create_input(),

      current: storage_builder.create_discrete_state(false),

      output: self.periph_output.unwrap(),

    }
  }
}

impl<'a> dscore::RequiresStorage for DigitalOutput<'a> {
  const SIZE: dscore::StorageSize = dscore::StorageSize {
    r_param: 0, b_param: 0, i_param: 0,
    r_out: 0, b_out: 0, i_out: 0,
    r_dstate: 0, b_dstate: 1, i_dstate: 0,
  };
}
