use flowmbed_dynsys::core::Float;

pub trait DigitalOutputPin {
    /// Drives the pin low
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
    /// electrical sources
    fn set_low(&mut self) -> anyhow::Result<()>;

    /// Drives the pin high
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
    /// electrical sources
    fn set_high(&mut self) -> anyhow::Result<()>;
}

pub trait PwmChannel {
  /// Disables a PWM `channel`
  fn disable(&mut self) -> anyhow::Result<()>;

  /// Enables a PWM `channel`
  fn enable(&mut self) -> anyhow::Result<()>;

  /// Returns the current duty cycle
  fn get_duty(&self) -> Float;

  /// Sets a new duty cycle
  fn set_duty(&mut self, duty: Float) -> anyhow::Result<()>;
}

pub trait PwmMultiChannel<const N: usize> {
  fn channel(&mut self, i: usize) -> anyhow::Result<&mut dyn PwmChannel>;
}