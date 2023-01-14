use flowmbed_dynsys::core as dscore;

/// Declare the block struct
#[allow(dead_code)]
pub struct DigitalOutput<'a> {

  pub input: dscore::Input<'a, bool>,

  pub current: dscore::DiscreteState<'a, bool>,

  pub out: crate::hal::OutputPin<'a>,
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> DigitalOutput<'a> {
  pub fn builder() -> Builder<'a> {
    Builder {
      __phantom: std::marker::PhantomData,
      periph_out: None,
    }
  }
}

pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  periph_out: Option<crate::hal::OutputPin<'a>>,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {

  pub fn out(mut self, v: crate::hal::OutputPin<'a>) -> Self {
    self.periph_out = Some(v);
    self
  }
}

impl<'a> dscore::BlockBuilder<'a, DigitalOutput<'a>> for Builder<'a> {
  fn build<ST: dscore::DefaultSystemStrorage>(self, storage_builder: &mut dscore::SystemStorageBuilder<'a, ST>) -> DigitalOutput<'a> {
    DigitalOutput {

      input: storage_builder.create_input(),

      current: storage_builder.create_discrete_state(false),

      out: self.periph_out.unwrap(),

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
