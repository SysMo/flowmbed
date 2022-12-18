use super::gpio;
// use core::sync::atomic::AtomicBool;
pub struct Peripherals {
  pub pins: gpio::Pins,
}

static mut TAKEN: bool = false;

impl Peripherals {
  pub fn take() -> anyhow::Result<Self> {
    if unsafe { TAKEN } {
      anyhow::bail!("Peripherals already taken")
    } else {
      unsafe { TAKEN = true };
      Ok(unsafe { Peripherals::new() })
    }
  }

  pub unsafe fn new() -> Self {
    Self {
      pins: gpio::Pins::new(),
    }
  }
}