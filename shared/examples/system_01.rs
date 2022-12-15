extern crate flowmbed_shared;

use const_default::ConstDefault;
use flowmbed_shared::dynsys::{system_storage as ss};
use flowmbed_shared::dynsys::system_storage::{SystemStorageBuilder, SystemStorageFacade, VariableCreator};
use flowmbed_shared::dynsys::variables::{Parameter, ContinuousState, DiscreteState};
use flowmbed_shared::dynsys::heap_storage::HeapSystemStorage;
use embedded_hal::digital;

type ST = HeapSystemStorage;

struct LedSystemVariables<'a> {
  t_on: Parameter<'a, f64>,
  t_off: Parameter<'a, f64>,
  led_on: DiscreteState<'a, bool>,
  last_change: DiscreteState<'a, i64>,
}

impl<'a> LedSystemVariables<'a> {
  pub fn new(storage: &'a ST) -> LedSystemVariables<'a> {
    let mut builder = 
      SystemStorageBuilder::new(storage);
    LedSystemVariables { 
      t_on: builder.create_param(2.0),
      t_off: builder.create_param(1.0), 
      led_on: builder.create_discrete_state(true),
      last_change: builder.create_discrete_state(0),
    }
  }
}

struct LedSystemPeripherals<'a> {
  led: &'a mut dyn digital::OutputPin<Error = anyhow::Error>
}

impl<'a> LedSystemPeripherals<'a> {
  pub fn new() -> LedSystemPeripherals<'a> {
    struct LED {}
    
    impl digital::ErrorType for LED {
      type Error = anyhow::Error;
    }
    
    impl digital::OutputPin for LED {

      fn set_high(&mut self) -> Result<(), Self::Error> {
        println!("LED is on");
        Ok(())
      }
      fn set_low(&mut self) -> Result<(), Self::Error> {
        println!("LED is off");
        Ok(())
      }
    }
    static mut led: LED = LED {};

    LedSystemPeripherals { 
      led: unsafe {&mut led}
    }
  }
}


struct LedSystem<'a> {
  variables: LedSystemVariables<'a>,
  peripherals: LedSystemPeripherals<'a>
}

impl<'a> LedSystem<'a> {
  pub fn new(storage: &'a ST) -> LedSystem<'a> {

    LedSystem {
      variables: LedSystemVariables::new(storage),
      peripherals: LedSystemPeripherals::new(),
    }
  }

  pub fn init(&self) -> anyhow::Result<()> {
    Ok(())
  }

  pub fn step(&mut self, t: u64) -> anyhow::Result<()> {
    let t_u = t as i64;
    let led_on = *self.variables.led_on;
    let last_t = *self.variables.last_change;
    if led_on && (t_u - last_t >= self.variables.t_on.round() as i64) {
      println!("Event at t = {}", t);
      self.variables.led_on.update(false);
      self.variables.last_change.update(t_u);
      self.peripherals.led.set_low();
    } else if !led_on && (t_u - last_t >= self.variables.t_off.round() as i64) {
      println!("Event at t = {}", t);
      self.variables.led_on.update(true);
      self.variables.last_change.update(t_u);
      self.peripherals.led.set_high();
    }

    Ok(())
  }
}

#[allow(unused_variables)]
fn main() -> anyhow::Result<()> {
  let size = ss::StorageSize {
    r_param: 2, b_dstate: 1, i_dstate: 1,
    ..ss::StorageSize::DEFAULT
  }; 
  let storage = 
    HeapSystemStorage::new(size);

  let mut system = LedSystem::new(&storage);
  let mut t: u64 = 0;
  let step = 1;

  while t <= 10 {
    system.step(t)?;
    std::thread::sleep(std::time::Duration::from_secs(step) / 5);
    t += step;
  }

  Ok(())
}