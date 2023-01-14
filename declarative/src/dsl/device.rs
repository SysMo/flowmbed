use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct PeripheralConfig {
  pub id: String,
  pub conf: PeripheralConfigEnum
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PeripheralConfigEnum {
  DigitalInput(DigitalInputConfig),
  DigitalOutput(DigitalOutputConfig),
  PWMOutput(PWMOutputConfig)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DigitalOutputConfig {
  pub pin: u32
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DigitalInputConfig {
  pub pin: u32
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PWMOutputConfig {
  pub pin: u32,
  pub channel: String,
  pub timer: String,
  pub freq: u64,  
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PeripheralRef(pub String);