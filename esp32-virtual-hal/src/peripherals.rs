use super::gpio;
// use core::sync::atomic::AtomicBool;
pub struct Peripherals {
  pub pins: gpio::Pins,
}

static mut TAKEN: bool = false;

impl Peripherals {
  pub fn take() -> Option<Self> {
    if unsafe { TAKEN } {
      None
    } else {
      unsafe { TAKEN = true };
      Some(unsafe { Peripherals::new() })
    }
  }

  pub unsafe fn new() -> Self {
    Self {
      pins: gpio::Pins::new(),
    }
  }
}