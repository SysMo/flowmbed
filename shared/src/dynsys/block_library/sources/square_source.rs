use crate::dynsys::{SystemStateInfo, StorageSize};
use crate::dynsys::system_storage::{SystemStorageBuilder, DefaultSystemStrorage};
use crate::dynsys::variables::{Parameter, DiscreteState, Output};
use crate::dynsys::Block;
use const_default::ConstDefault;

pub struct SquareSource<'a> {
  period: Parameter<'a, f64>,
  duty: Parameter<'a, f64>,
  initial: Parameter<'a, bool>,
  current: DiscreteState<'a, bool>,
  last_change: DiscreteState<'a, f64>,
  pub output: Output<'a, bool>,
}

// pub struct StateUpdate {
//   current: Option<bool>
// }
// pub struct Outputs {

// }

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

  pub fn init(&mut self) {
    self.current.update(*self.initial);
  }

  pub fn compute(&self, ssi: &SystemStateInfo) -> anyhow::Result<()> {
    let tau_on = *self.period *  *self.duty;
    let tau_off = *self.period - tau_on;
    // Update state
    if *self.current && (ssi.t - *self.last_change >= tau_on) {
      println!("Event at t = {}", ssi.t);
      self.current.update(false);
      self.last_change.update(ssi.t);
    } else if !*self.current && (ssi.t - *self.last_change >= tau_off) {
      println!("Event at t = {}", ssi.t);
      self.current.update(true);
      self.last_change.update(ssi.t);
    }
    // Update outputs
    self.output.update(*self.current);

    Ok(())
  }

}

impl<'a> Block for SquareSource<'a> {
    const size: StorageSize = StorageSize {
      r_param: 2, b_param: 1, b_dstate: 1, r_dstate: 1, b_out: 1,
      ..StorageSize::DEFAULT
    };
}

// period: Parameter<'a, f64>,
// duty: Parameter<'a, f64>,
// initial: Parameter<'a, bool>,
// current: DiscreteState<'a, bool>,
// last_change: DiscreteState<'a, f64>,
// output: Output<'a, bool>,