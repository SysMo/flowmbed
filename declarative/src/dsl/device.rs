use serde::{Serialize, Deserialize};
use crate::gen::device::{DeviceConfigGenerator, PeripheralConfigGenerator};
use genco::prelude::{rust, quote};
// Check out https://github.com/dtolnay/typetag
// For for possibly implementing trait loading from YAML

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Device {
  pub id: String,
  pub config: Box<dyn DeviceConfig>
}

impl Device {
  pub fn gen<'a>(&'a self) -> &'a dyn DeviceConfigGenerator {
    self.config.as_ref()
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceRef(pub String);

#[typetag::serde(tag = "type")]
pub trait DeviceConfig: DeviceConfigGenerator + core::fmt::Debug {
  fn peripherals<'a>(&'a self) -> &'a Vec<Peripheral>;
  // fn gen<'a>(&'a self) -> &'a dyn DeviceConfigGenerator;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Peripheral {
  pub id: String,
  pub config: Box<dyn PeripheralConfig>
}

impl Peripheral {
  pub fn gen<'a>(&'a self) -> &'a dyn PeripheralConfigGenerator {
    self.config.as_ref()
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PeripheralRef(pub String);

#[typetag::serde(tag = "type")]
pub trait PeripheralConfig: PeripheralConfigGenerator + core::fmt::Debug {
  // fn gen<'a>(&'a self) -> &'a dyn PeripheralConfigGenerator;
}
