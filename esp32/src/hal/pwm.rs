use esp_idf_hal::ledc;
use esp_idf_hal::gpio;
use esp_idf_hal::peripheral::Peripheral;
use flowmbed_peripherals::actuators::traits as actuators;
use flowmbed_dynsys::core::{Float};
use core::borrow::Borrow;
use log::*;

pub struct Esp32Ledc<'a> {
  driver: ledc::LedcDriver<'a>
}

impl<'a> Esp32Ledc<'a> {
  pub fn new<C: ledc::LedcChannel, B: Borrow<ledc::LedcTimerDriver<'a>>>(
    _channel: impl Peripheral<P = C> + 'a,
    timer_driver: B,
    pin: impl Peripheral<P = impl gpio::OutputPin> + 'a,
  ) -> anyhow::Result<Self> {
    Ok(Esp32Ledc { 
      driver: ledc::LedcDriver::new(
      _channel, timer_driver, pin
    )?})
  }
}

impl<'a> actuators::PwmChannel for Esp32Ledc<'a> {
  /// Disables a PWM `channel`
  fn disable(&mut self) -> anyhow::Result<()> {
    self.driver.disable().map_err(|x| x.into())
  }

  /// Enables a PWM `channel`
  fn enable(&mut self) -> anyhow::Result<()> {
    self.driver.enable().map_err(|x| x.into())
  }

  /// Returns the current duty cycle
  fn get_duty(&self) -> Float {
    (self.driver.get_duty() as Float) / (self.driver.get_max_duty() as Float)
  }

  /// Sets a new duty cycle
  fn set_duty(&mut self, duty: Float) -> anyhow::Result<()> {
    let duty_u32 = (duty * (self.driver.get_max_duty() as Float)).round() as u32;
    self.driver.set_duty(duty_u32).map_err(|x| x.into())
  }
}

pub struct Esp32LedcMultiChannel<'a, const N: usize> {
  // timer_driver: ledc::LedcTimerDriver<'a>,
  channel_drivers: [Option<Esp32Ledc<'a>>; N],
}


use esp_idf_hal::prelude::Hertz;

impl<'a, const N: usize> Esp32LedcMultiChannel<'a, N> {
  pub fn builder(
    f: Hertz,
    timer: impl Peripheral<P = impl ledc::LedcTimer> + 'a,
  ) -> anyhow::Result<Esp32LedcMultiChannelBuilder<'a, N>> {

    let timer_config = 
      ledc::config::TimerConfig::new().frequency(f);

    let timer_driver = ledc::LedcTimerDriver::new(
      timer, &timer_config
    )?;

    const INIT: Option<Esp32Ledc> = None;
    let drivers: [Option<Esp32Ledc<'a>>; N] = [INIT; N];
  
    Ok(Esp32LedcMultiChannelBuilder {
      timer_driver,
      channel_drivers: drivers,
      n: 0, 
    })
    
  }

}

pub struct Esp32LedcMultiChannelBuilder<'a, const N: usize> {
  timer_driver: ledc::LedcTimerDriver<'a>,
  channel_drivers: [Option<Esp32Ledc<'a>>; N],
  n: usize,
}

impl<'a, const N: usize> Esp32LedcMultiChannelBuilder<'a, N> {
  pub fn add_channel(
    mut self,
    ch: impl Peripheral<P = impl ledc::LedcChannel> + 'a,
    pin: impl Peripheral<P = impl gpio::OutputPin> + 'a,
  ) -> anyhow::Result<Self> {
    if self.n < N {
      self.channel_drivers[self.n] = Some(
        Esp32Ledc::new(
          ch, &self.timer_driver, pin
        )?        
      );
      self.n += 1;
      Ok(self)
    } else {
      anyhow::bail!("Channel capacity ({}) exceeded!", N)
    }
  }

  pub fn build(self) -> anyhow::Result<Esp32LedcMultiChannel<'a, N>> {
    if self.n < N {
      anyhow::bail!("Only {} out of {} PWM channels configured!", self.n, N)
    } else {
      info!("Building the Esp32LedcMultiChannel");
      Ok(Esp32LedcMultiChannel {
        // timer_driver: self.timer_driver,
        channel_drivers: self.channel_drivers
      })  
    }
  }
}


impl<'a, const N: usize> actuators::PwmMultiChannel<N> 
for Esp32LedcMultiChannel<'a, N> {
  fn channel(&mut self, i: usize) -> anyhow::Result<&mut dyn actuators::PwmChannel> {
    let channel = self.channel_drivers[i].as_mut()
      .ok_or(anyhow::anyhow!("Channel not initialized"))?;
    Ok(channel)
  }
}



