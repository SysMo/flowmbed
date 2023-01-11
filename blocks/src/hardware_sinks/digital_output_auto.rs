use flowmbed_dynsys::core as dscore;

#[doc=" Declare the block struct"]
#[allow(dead_code)]
pub struct DigitalOutput<'a> {

  pub input: dscore::Input<'a, dscore::Bool>,

  pub current: dscore::DiscreteState<'a, dscore::Bool>,

  pub out: crate::hal::OutputPin<'a>,
}

#[doc=" Implement the block struct"]
#[allow(dead_code)]
impl<'a> DigitalOutput<'a> {

  pub fn builder<ST: dscore::DefaultSystemStrorage>(
    storage_builder: &'a mut dscore::SystemStorageBuilder<'a, ST>
  ) -> BlockBuilder<'a, ST> {
    BlockBuilder {
      __storage_builder: storage_builder,
      periph_out: None,
    }
  }
}

pub struct BlockBuilder<'a, ST: dscore::DefaultSystemStrorage> {
  __storage_builder: &'a mut dscore::SystemStorageBuilder<'a, ST>,
  periph_out: Option<crate::hal::OutputPin<'a>>,
}

#[allow(dead_code)]
impl<'a, ST: dscore::DefaultSystemStrorage> BlockBuilder<'a, ST> {

  pub fn out(mut self, v: crate::hal::OutputPin<'a>) -> Self {
    self.periph_out = Some(v);
    self
  }
}

impl<'a, ST: dscore::DefaultSystemStrorage> From<BlockBuilder<'a, ST>> for DigitalOutput<'a> {
  fn from(builder: BlockBuilder<'a, ST>) -> Self {
    DigitalOutput {

      input: builder.__storage_builder.create_input(),

      current: builder.__storage_builder.create_discrete_state(false),

      out: builder.periph_out.unwrap(),

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
