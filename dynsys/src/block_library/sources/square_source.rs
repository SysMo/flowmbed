use crate::core::{
  Block,
  SystemStateInfo, StorageSize, DefaultSystemStrorage,
  SystemStorageBuilder,
  Parameter, DiscreteState, Output,
};
use const_default::ConstDefault;

pub struct SquareSource<'a> {
  pub period: Parameter<'a, f64>,
  pub duty: Parameter<'a, f64>,
  pub initial: Parameter<'a, bool>,

  pub output: Output<'a, bool>,

  current: DiscreteState<'a, bool>,
  last_change: DiscreteState<'a, f64>,
}

impl<'a> SquareSource<'a> {
  pub fn new<ST: DefaultSystemStrorage>(
    builder: &mut SystemStorageBuilder<'a, ST>
  ) -> SquareSource<'a> {
    SquareSource { 
      period: builder.create_param(1.0),
      duty: builder.create_param(0.5),
      initial: builder.create_param(true),
      current: builder.create_discrete_state(false),
      last_change: builder.create_discrete_state(0.0),
      output: builder.create_output(false),
    }
  }

  pub fn init(&mut self) -> anyhow::Result<()> {
    self.current.initialize(*self.initial);
    self.output.initialize(*self.initial);
    Ok(())
  }

  pub fn compute(&self, ssi: &SystemStateInfo) -> anyhow::Result<()> {
    let tau_on = *self.period *  *self.duty;
    let tau_off = *self.period - tau_on;
    // Update state
    if *self.current && (ssi.t - *self.last_change >= tau_on) {
      self.current.update(false, ssi);
      self.last_change.update(ssi.t, ssi);
    } else if !*self.current && (ssi.t - *self.last_change >= tau_off) {
      self.current.update(true, ssi);
      self.last_change.update(ssi.t, ssi);
    }
    // Update outputs
    self.output.update(*self.current, ssi);

    Ok(())
  }

}

impl<'a> Block for SquareSource<'a> {
    const BLOCK_SIZE: StorageSize = StorageSize {
      r_param: 2, b_param: 1, b_dstate: 1, r_dstate: 1, b_out: 1,
      ..StorageSize::DEFAULT
    };
}
