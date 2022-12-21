use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfig {
  pub id: String,
  pub kind: DeviceKind,
  pub peripherals: Vec<PeripheralConfig>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DeviceKind {
    esp32
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PeripheralConfig {
  pub id: String,
  pub conf: PeripheralConfigEnum
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PeripheralConfigEnum {
  DigitalInput(DigitalInputConfig),
  DigitalOutput(DigitalOutputConfig)
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalOutputConfig {
  pub pin: u32
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalInputConfig {
  pub pin: u32
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PeripheralRef(pub String);