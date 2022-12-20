use crate::core::{
  Block,
  SystemStateInfo, StorageSize, DefaultSystemStrorage,
  SystemStorageBuilder,
  Parameter, DiscreteState, Output, Input,
};
use const_default::ConstDefault;
use crate::util::debug::materialize_vars;
use crate::util::block_macros::*;

pub struct CountingTrigger<'a> {
  pub pulses_up:  Parameter<'a, i64>,
  pub pulses_down:  Parameter<'a, i64>,
  pub initial_state:  Parameter<'a, bool>,
  pub initial_count:  Parameter<'a, i64>,
  pub count_on_rising: Parameter<'a, bool>,

  pub input: Input<'a, bool>,
  pub output: Output<'a, bool>,

  last_input: DiscreteState<'a, bool>,
  current: DiscreteState<'a, bool>,
  counter: DiscreteState<'a, i64>,
}

block_builder!(CountingTrigger, 
  [pulses_up, Parameter, i64]
  [pulses_down, Parameter, i64]
  [initial_count, Parameter, i64]
);


impl <'a> CountingTrigger<'a> {
  pub fn new<ST: DefaultSystemStrorage>(
    builder: &mut SystemStorageBuilder<'a, ST>
  ) -> CountingTrigger<'a> {
    CountingTrigger {
      pulses_up: builder.create_param(1),
      pulses_down: builder.create_param(1),
      initial_state: builder.create_param(false),
      initial_count: builder.create_param(0),
      count_on_rising: builder.create_param(true),

      input: builder.create_input(),
      output: builder.create_output(false),
      
      last_input: builder.create_discrete_state(false),
      current: builder.create_discrete_state(false),
      counter: builder.create_discrete_state(0)
    }
  }

  pub fn builder<ST: DefaultSystemStrorage>(
    builder: &mut SystemStorageBuilder<'a, ST>
  ) -> Builder<'a> {
    Builder { component: Self::new(builder) }
  }

  pub fn init(&mut self) -> anyhow::Result<()> {
    self.current.initialize(*self.initial_state);
    self.output.initialize(*self.initial_state);
    self.counter.initialize(*self.initial_count);
    Ok(())
  }

  pub fn compute(&self, ssi: &SystemStateInfo) -> anyhow::Result<()> {
    materialize_vars!(self,
      pulses_up, pulses_down,
      count_on_rising, input, last_input, counter, current
    );

    if *self.input != *self.last_input {
      if (*self.input && *self.count_on_rising) ||  (!*self.input && !*self.count_on_rising) {
        self.counter.update(*self.counter + 1, ssi);
      }
      self.last_input.update(*self.input, ssi);
    }

    if *self.current && (*self.counter >= *self.pulses_down) {      
      self.counter.update(0, ssi);
      self.current.update(false, ssi);
    } else if !*self.current && (*self.counter >= *self.pulses_up) {
      self.counter.update(0, ssi);
      self.current.update(true, ssi);
    }

    self.output.update(*self.current, ssi);
    Ok(())
  }
}


impl<'a> Block for CountingTrigger<'a> {
  const BLOCK_SIZE: StorageSize = StorageSize {
    i_param: 3, b_param: 2, b_dstate: 2, i_dstate: 1, b_out: 1,
    ..StorageSize::DEFAULT
  };
}