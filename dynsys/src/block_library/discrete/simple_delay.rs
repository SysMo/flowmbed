use crate::core::{
  Block,
  SystemStateInfo, StorageSize, DefaultSystemStrorage,
  SystemStorageBuilder,
  Parameter, DiscreteState, Output, Input,
};
use const_default::ConstDefault;

pub struct SimpleDelay<'a> {
  pub delay: Parameter<'a, f64>,
  pub initial_output: Parameter<'a, bool>,
  
  pub input: Input<'a, bool>,
  pub output: Output<'a, bool>,

  future: DiscreteState<'a, bool>,
  current: DiscreteState<'a, bool>,
  last_input_change: DiscreteState<'a, f64>,
}

impl<'a> SimpleDelay<'a> {
  pub fn new<ST: DefaultSystemStrorage>(
    builder: &mut SystemStorageBuilder<'a, ST>
  ) -> SimpleDelay<'a> {
    SimpleDelay { 
      delay: builder.create_param(0.2),
      initial_output: builder.create_param(false),
      future: builder.create_discrete_state(false),
      current: builder.create_discrete_state(false),
      last_input_change: builder.create_discrete_state(0.0),
      input: builder.create_input(),
      output: builder.create_output(false),
    }
  }

  pub fn init(&mut self) -> anyhow::Result<()> {
    self.future.initialize(*self.input);
    self.current.initialize(*self.initial_output);
    self.output.initialize(*self.initial_output);
    Ok(())
  }

  pub fn compute(&self, ssi: &SystemStateInfo) -> anyhow::Result<()> {
    if *self.input != *self.future {
      self.future.update(*self.input, ssi);
      self.last_input_change.update(ssi.t, ssi);
    }
    if (*self.future != *self.current) && (ssi.t - *self.last_input_change >= *self.delay) {
      self.current.update(*self.future, ssi);
    }
    // Update outputs
    self.output.update(*self.current, ssi);

    Ok(())
  }

}

impl<'a> Block for SimpleDelay<'a> {
    const BLOCK_SIZE: StorageSize = StorageSize {
      r_param: 1, b_param: 1, b_dstate: 2, r_dstate: 1, b_out: 1,
      ..StorageSize::DEFAULT
    };
}
