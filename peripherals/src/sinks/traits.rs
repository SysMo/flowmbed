// use embedded_hal::digital;
// use embedded_hal_0_2 as eh_02;
// use crate::error::ErrorType;
// use 

// pub trait FloatSink {
//   fn send(&mut self, v: f64) -> anyhow::Result<()>;
// }
use flowmbed_dynsys::data::Value;

pub trait ValueSink {
  fn send(&mut self, v: Value) -> anyhow::Result<()>;
}