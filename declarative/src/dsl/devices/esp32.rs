use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ESP32Device {
  pub peripherals: Vec<PeripheralConfig>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeripheralConfig {
  pub id: String,
  pub config: PeripheralConfigEnum
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PeripheralConfigEnum {
  DigitalInputPin(DigitalInputPinConfig),
  DigitalOutputPin(DigitalOutputPinConfig),
  PwmPin(PwmPinConfig)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DigitalOutputPinConfig {
  pub pin: u32
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DigitalInputPinConfig {
  pub pin: u32
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PwmPinConfig {
  pub pin: u32,
  pub channel: String,
  pub timer: String,
  pub freq: u64,  
}