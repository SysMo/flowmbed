use flowmbed_dynsys::core as dscore;

/// Declare the block struct
#[allow(dead_code)]
pub struct PWMOutput<'a> {

  pub duty: dscore::Input<'a, f64>,

  pub out: crate::hal::PwmPin<'a>,
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
  periph_out: Option<crate::hal::PwmPin<'a>>,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {

  pub fn out(mut self, v: crate::hal::PwmPin<'a>) -> Self {
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
