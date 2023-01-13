use std::any;

use serde::{Serialize, Deserialize};
use super::device::DeviceConfig;
use super::circuit::CircuitConfig;
use super::task::TaskConfigEnum;
use super::references::PackageImport;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SystemConfig {
  pub imports: Vec<PackageImport>,
  pub devices: Vec<DeviceConfig>,
  pub circuits: Vec<CircuitConfig>,
  pub tasks: Vec<TaskConfigEnum>,
}



