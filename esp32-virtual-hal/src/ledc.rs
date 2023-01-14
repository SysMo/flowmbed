// use std::marker::PhantomData;
// use core::borrow::Borrow;
// use crate::peripheral::Peripheral;
// use crate::gpio::OutputPin;
// use crate::HalError;

// // type Duty = u32;
// type Duty = f64;
// type HPoint = Duty;

// #[derive(PartialEq, Eq, Copy, Clone, Debug)]
// pub enum SpeedMode {
//     #[cfg(esp_idf_soc_ledc_support_hs_mode)]
//     /// High Speed Mode. Currently only supported on the ESP32.
//     HighSpeed,
//     /// Low Speed Mode. The only configuration supported on ESP32S2, ESP32S3, ESP32C2 and ESP32C3.
//     LowSpeed,
// }

// impl Default for SpeedMode {
//   fn default() -> Self {
//       Self::LowSpeed
//   }
// }

// /// LED Control peripheral timer
// pub trait LedcTimer {
//   fn timer() -> &'static str;
// }

//   /// LED Control peripheral output channel
// pub trait LedcChannel {
//   fn channel() -> &'static str;
// }

// pub struct LedcTimerDriver<'d> {
//   timer: u8,
//   speed_mode: SpeedMode,
//   max_duty: Duty,
//   _p: PhantomData<&'d mut ()>,
// }

// impl<'d> LedcTimerDriver<'d> {
//   pub fn new<T: LedcTimer>(
//     _timer: impl Peripheral<P = T> + 'd,
//     config: &config::TimerConfig,
//   ) -> Result<Self, HalError> {
//     Ok(Self {
//       timer: T::timer() as _,
//       speed_mode: config.speed_mode,
//       max_duty: config.resolution.max_duty(),
//       _p: PhantomData,
//     })
//   }
// }


// pub struct LedcDriver<'d> {
//   channel: u8,
//   timer: u8,
//   duty: Duty,
//   hpoint: HPoint,
//   speed_mode: SpeedMode,
//   max_duty: Duty,
//   _p: PhantomData<&'d mut ()>,
// }

// impl<'d> LedcDriver<'d> {
//   /// Creates a new LED Control driver
//   pub fn new<C: LedcChannel, B: Borrow<LedcTimerDriver<'d>>>(
//       _channel: impl Peripheral<P = C> + 'd,
//       timer_driver: B,
//       pin: impl Peripheral<P = impl OutputPin> + 'd,
//   ) -> Result<Self, HalError> {
//     let duty = 0.0;
//     let hpoint = 0.0;

//     Ok(LedcDriver {
//       duty,
//       hpoint,
//       speed_mode: timer_driver.borrow().speed_mode,
//       max_duty: timer_driver.borrow().max_duty,
//       timer: timer_driver.borrow().timer() as _,
//       channel: C::channel() as _,
//       _p: PhantomData,
//   }) 
//   }}