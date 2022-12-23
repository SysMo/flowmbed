use serde::{Serialize, Deserialize};
use super::device::DeviceConfig;
use super::circuit::CircuitConfig;
use super::task::TaskConfigEnum;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SystemConfig {
  pub devices: Vec<DeviceConfig>,
  pub circuits: Vec<CircuitConfig>,
  pub tasks: Vec<TaskConfigEnum>,
}