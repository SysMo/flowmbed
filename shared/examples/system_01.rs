extern crate flowmbed_shared;
use std::borrow::Borrow;

use const_default::ConstDefault;
use flowmbed_shared::dynsys::{system_storage as ss};
use flowmbed_shared::dynsys::system_storage::{SystemStorageBuilder, SystemStorageFacade, VariableCreator, HeapSystemStorage};
use flowmbed_shared::dynsys::variables::{Parameter, ContinuousState, DiscreteState};


struct LedSystem<'a> {
  t_on: Parameter<'a, f64>,
  t_off: Parameter<'a, f64>,
  led_on: DiscreteState<'a, bool>,
}

impl<'a> LedSystem<'a> {
  pub fn new(storage: &'a HeapSystemStorage) -> LedSystem<'a> {
    let mut builder = 
      SystemStorageBuilder::new(storage);
    LedSystem { 
      t_on: builder.create(), 
      t_off: builder.create(), 
      led_on: builder.create(), 
    }

  }
}

#[allow(unused_variables)]
fn main() -> anyhow::Result<()> {
  let size = ss::StorageSize {
    r_param: 2, b_state: 1,
    ..ss::StorageSize::DEFAULT
  }; 
  let storage = 
    ss::HeapSystemStorage::new(size);

  // let mut builder = 
  //   SystemStorageBuilder::new(&storage);
  // let t_on: Parameter<f64> = builder.create();
  // let t_off: Parameter<f64> = builder.create();
  // let led_on: DiscreteState<bool> = builder.create();

  let system = LedSystem::new(&storage);
  
  storage.set_parameter(system.t_on.id, 1.0)?;
  storage.set_parameter(system.t_off.id, 2.0)?;
  storage.set_discrete_state(system.led_on.id, true)?;
  println!("{:#?}", storage);
  println!("{}", *system.t_on);
  Ok(())
}