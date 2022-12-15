use crate::dynsys::system_storage::{SystemStorageBuilder, DefaultSystemStrorage};
use crate::dynsys::variables::{Parameter, DiscreteState};

pub struct SquareSource<'a> {
  freq: Parameter<'a, f64>,
  duty: Parameter<'a, f64>,
  initial: Parameter<'a, bool>,
  state: DiscreteState<'a, bool>,
  last_change: DiscreteState<'a, i64>,
}

impl<'a> SquareSource<'a> {
  pub fn new<ST: DefaultSystemStrorage>(builder: &'a mut SystemStorageBuilder<'a, ST>) -> SquareSource<'a> {
    SquareSource { 
      freq: builder.create_param(1.0),
      duty: builder.create_param(0.5),
      initial: builder.create_param(true),
      state: builder.create_discrete_state(false),
      last_change: builder.create_discrete_state(0)
    }
  }
}