use embedded_hal::digital;
use log::*; 
use std::marker::PhantomData;
use super::peripheral::{Peripheral, PeripheralRef};

type HalError = anyhow::Error;

pub trait Pin: Peripheral<P = Self> + Sized + Send + 'static {
  fn pin(&self) -> i32;
}

pub struct Input;
pub struct Output;
pub struct InputOutput;

pub trait OutputPin: Pin {

}

pub trait OutputMode {

}

impl OutputMode for Output {

}

pub struct PinDriver<'d, T: Pin, MODE> {
  pin: PeripheralRef<'d, T>,
  _mode: PhantomData<MODE>,
}

impl<'d, T: OutputPin> PinDriver<'d, T, Output> {
  /// Creates the driver for a pin in output state.
  #[inline]
  pub fn output(pin: impl Peripheral<P = T> + 'd) -> Result<Self, HalError> {
      crate::into_ref!(pin);

      Ok(Self {
          pin,
          _mode: PhantomData,
      })
  }
}

impl<'d, T: Pin, MODE> PinDriver<'d, T, MODE> {
  /// Returns the pin number.
  pub fn pin(&self) -> i32 {
      self.pin.pin()
  }

}

impl<'d, T: Pin, MODE> digital::ErrorType for PinDriver<'d, T, MODE> {
  type Error = anyhow::Error;
}

impl<'d, T: Pin, MODE> digital::OutputPin for PinDriver<'d, T, MODE> 
where MODE : OutputMode {
  fn set_high(&mut self) -> Result<(), Self::Error> {
    info!("LED {} is on", self.pin.pin());
    Ok(())
  }
  fn set_low(&mut self) -> Result<(), Self::Error> {
    info!("LED {} is off", self.pin.pin());
    Ok(())
  }
}



macro_rules! any_pin {
  ($pxi:ident: $pin:expr) => {
    crate::impl_peripheral!($pxi);

    impl Pin for $pxi {
      fn pin(&self) -> i32 {
          $pin
      }
    }
  };
}

macro_rules! output_pin {
  ($pxi:ident: $pin:expr) => {
    impl OutputPin for $pxi {}
  };
}

macro_rules! pin {
  ($pxi:ident: $pin:expr, Output) => {
    any_pin!($pxi: $pin);
    output_pin!($pxi: $pin);
  };
}



pin!(Gpio2:2, Output);
pin!(Gpio4:4, Output);

pub struct Pins {
  pub gpio2: Gpio2,
  pub gpio4: Gpio4,
}

impl Pins {
  pub unsafe fn new() -> Pins {
    Pins {
      gpio2: Gpio2::new(),
      gpio4: Gpio4::new(),
    }
  }
}
