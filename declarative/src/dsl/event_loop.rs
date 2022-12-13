use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EventLoopConfigEnum {
  FixedIntervalLoop(FixedIntervalLoopConfig)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FixedIntervalLoopConfig {
  interval: u32,
  unit: IntervalUnit
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum IntervalUnit {
  ms, us
}