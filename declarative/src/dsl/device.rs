use serde::{Serialize, Deserialize};
use super::devices;


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