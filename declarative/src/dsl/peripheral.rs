use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PeripheralConfig {
  id: String,
  conf: PeripheralConfigEnum
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PeripheralConfigEnum {
  DigitalInput(DigitalInputConfig),
  DigitalOutput(DigitalOutputConfig)
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalOutputConfig {
  pin: u32
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalInputConfig {
  pin: u32
}