use serde::{Serialize, Deserialize};
use super::devices;


// Check out https://github.com/dtolnay/typetag
// For for possibly implementing trait loading from YAML

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeviceConfig {
  pub id: String,
  pub config: DeviceConfigEnum
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum DeviceConfigEnum {
  ESP32(devices::ESP32Device),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PeripheralRef(pub String);

pub trait IDeviceConfig {}

pub trait IPeripheralConfig {}