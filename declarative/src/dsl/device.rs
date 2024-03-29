use serde::{Serialize, Deserialize};
use crate::{gen::device::{DeviceConfigGenerator, PeripheralConfigGenerator}};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Device {
  pub id: String,
  pub config: Box<dyn DeviceConfig>
}

// impl Device {
//   pub fn gen<'a>(&'a self) -> &'a dyn DeviceConfigGenerator {
//     self.config.as_ref()
//   }
// }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceRef(pub String);

#[typetag::serde(tag = "type")]
pub trait DeviceConfig: DeviceConfigGenerator + core::fmt::Debug {
  fn peripherals<'a>(&'a self) -> &'a Vec<Peripheral>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Peripheral {
  pub id: String,
  pub config: Box<dyn PeripheralConfig>
}

// impl Peripheral {
//   pub fn gen<'a>(&'a self, context: &'a GenerationContext<'a>) -> PeripheralGenerator<'a> {
//     PeripheralGenerator {
//       context: context.push(&self.id, ContextObject::peripheral(self.config.as_ref())),
//       config: self.config.as_ref()
//     }    
//   }
// }


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PeripheralRef(pub String);

#[typetag::serde(tag = "type")]
pub trait PeripheralConfig: PeripheralConfigGenerator + core::fmt::Debug + core::any::Any {

}
