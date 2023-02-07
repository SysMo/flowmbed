use const_default::ConstDefault;
use serde::{Serialize, Deserialize};
use super::device::Device;
use super::circuit::CircuitConfig;
use super::task::TaskConfigEnum;
use super::references::PackageImport;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SystemConfig {
  pub imports: Vec<PackageImport>,
  pub devices: Vec<Device>,
  pub circuits: Vec<CircuitConfig>,
  pub tasks: Vec<TaskConfigEnum>,
}


impl ConstDefault for SystemConfig {
  const DEFAULT: Self = SystemConfig{
    imports: Vec::new(), devices: Vec::new(),
    circuits: Vec::new(), tasks: Vec::new()
  };
}