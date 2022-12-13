use std::time::Duration;
use embedded_hal::digital::v2::{OutputPin};
use anyhow;

pub trait DynamicalSystem {
  fn init(self);
  fn step(self, t: Duration);
}

pub struct MainSystem<'a> {
  peripherals: SystemPeripherals<'a>,
}

// impl<'a> MainSystem<'a> {
//   pub fn new() -> MainSystem<'a> {
//     MainSystem { peripherals: SystemPeripherals::new() }
//   }
// }

pub struct SystemPeripherals<'a> {
  led_out: &'a dyn OutputPin<Error = anyhow::Error>,
}

// impl<'a> SystemPeripherals<'a> {
//   pub fn new() -> SystemPeripherals<'a> {
//     struct LedOut ();
//     impl OutputPin for LedOut {
//       type Error = anyhow::Error;
//     }


//     SystemPeripherals { led_out:  LedOut }
//   }
// }

struct LedOut ();
impl OutputPin for LedOut {
  type Error = anyhow::Error;
  
  fn set_low(&mut self) -> anyhow::Result<()> {
    println!("LED off");
    Ok(())
  }

  fn set_high(&mut self) -> anyhow::Result<()> {
    println!("LED on");
    Ok(())
  }
}

pub fn main() -> anyhow::Result<()> {
  let led_out = LedOut ();


  let system = MainSystem {
    peripherals: SystemPeripherals { 
      led_out: &led_out
    } 
  };
  Ok(())
}