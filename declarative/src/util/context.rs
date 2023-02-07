use crate::dsl::{device::{Device, Peripheral}, system::SystemConfig};
use super::QualifiedPath;
use core::any::Any;

pub fn system_context<'a>(config: &'a SystemConfig) -> SystemContext<'a> {
  SystemContext::new(config)
}

pub fn empty_context<'a>() -> EmptyContext {
  EmptyContext {  }
}

pub trait GenerationContext {
  fn id(&self) -> &str;
  fn qpath(&self) -> &QualifiedPath;
  fn parent(&self) -> Option<&dyn GenerationContext>;

  fn as_system_context(&self) -> anyhow::Result<&SystemContext> {
    anyhow::bail!("{} not a system!", self.qpath().join("."))
  }

  fn as_device_context(&self) -> anyhow::Result<&DeviceContext> {
    anyhow::bail!("{} not a device!", self.qpath().join("."))
  }

  fn as_peripheral_context(&self) -> anyhow::Result<&PeripheralContext> {
    anyhow::bail!("{} not a peripheral!", self.qpath().join("."))
  }


  fn find_device(&self) -> anyhow::Result<&DeviceContext> {
    match self.as_device_context() {
      Ok(x) => Ok(x),
      _ => match self.parent() {
        Some(parent) => parent.find_device(),
        None => anyhow::bail!("Could not find device context"),
      }
    }
  }

}

pub struct EmptyContext {

}

impl EmptyContext {

}

impl<'a> GenerationContext for EmptyContext {
  fn id(&self) -> &str {""}
  fn parent(&self) -> Option<&dyn GenerationContext> { None }
  fn qpath(&self) -> &QualifiedPath{
    QualifiedPath::empty_ref()
  }
}

pub struct SystemContext<'a> {
  pub system: &'a SystemConfig,
}

impl<'a> SystemContext<'a> {
  // const empty_qpath: QualifiedPath = QualifiedPath::empty();
  pub fn new(config: &'a SystemConfig) -> SystemContext {
    SystemContext { system: config}
  }

  pub fn push_device(&'a self, device: &'a Device) -> DeviceContext<'a> {
    DeviceContext { device, _parent: Some(self), _qpath: self.qpath().append(&device.id), }
  }

}

impl<'a> GenerationContext for SystemContext<'a> {
  fn id(&self) -> &str {
    ""
  }
  fn parent(&self) -> Option<&dyn GenerationContext> {
    None
  }
  fn qpath(&self) -> &QualifiedPath{
    QualifiedPath::empty_ref()
  }

  fn as_system_context(&self) -> anyhow::Result<&SystemContext> {
    Ok(self)
  }
}

pub struct DeviceContext<'a> {
  pub device: &'a Device,
  pub _parent: Option<&'a dyn GenerationContext>,
  pub _qpath: QualifiedPath,
}

impl<'a> DeviceContext<'a> {
  pub fn push_peripheral(&'a self, peripheral: &'a Peripheral) -> PeripheralContext<'a> {
    PeripheralContext { peripheral, _parent: Some(self), _qpath: self.qpath().append(&peripheral.id), }
  }

  pub fn var_internal_periph(&self) -> &'a str {
    &"internal_peripherals"
  }

  pub fn var_device_periph(&self) -> &'a str {
    // &"device_peripherals"
    &"self"
  }
}

impl<'a> GenerationContext for DeviceContext<'a> {
  fn id(&self) -> &str {
      &self.device.id
  }

  fn parent(&self) -> Option<&dyn GenerationContext> {
      self._parent
  }

  fn qpath(&self) -> &QualifiedPath {
      &self._qpath
  }

  fn as_device_context(&self) -> anyhow::Result<&DeviceContext> {
    Ok(self)
  }
}

pub struct PeripheralContext<'a> {
  pub peripheral: &'a Peripheral,
  pub _parent: Option<&'a dyn GenerationContext>,
  pub _qpath: QualifiedPath,
}

impl<'a> PeripheralContext<'a> {
  pub fn push_peripheral(&'a self, peripheral: &'a Peripheral) -> PeripheralContext<'a> {
    PeripheralContext { peripheral, _parent: Some(self), _qpath: self.qpath().append(&peripheral.id), }
  }

  pub fn long_id(&'a self) -> String {
    self.qpath().relative_path(
      &self.find_device().unwrap().qpath()
    ).unwrap().join("_")
  }

  pub fn parent_peripheral_config_as<T: 'static>(&self) -> anyhow::Result<&T> {
    Ok((self.parent()
      .ok_or_else(|| anyhow::anyhow!("peripheral should have a parent"))
      .and_then(|x| x.as_peripheral_context())?.peripheral.config.as_ref() as &dyn Any).downcast_ref::<T>()
      .ok_or_else(|| anyhow::anyhow!("peripheral parent not of the requested type"))?)
  }
}


impl<'a> GenerationContext for PeripheralContext<'a> {
  fn id(&self) -> &str {
      &self.peripheral.id
  }

  fn parent(&self) -> Option<&'a dyn GenerationContext> {
      self._parent
  }

  fn qpath(&self) -> &QualifiedPath {
      &self._qpath
  }

  fn as_peripheral_context(&self) -> anyhow::Result<&PeripheralContext> {
    Ok(self)
  }
}


// pub enum GenerationContext<'a> {
//   Empty,
//   System(&'a SystemContext<'a>),
//   Device(&'a DeviceContext<'a>),
//   Peripheral(&'a PeripheralContext<'a>)
// }


// impl<'a> GenerationContext<'a> {

//   pub fn empty() -> GenerationContext<'a> {
//     GenerationContext::Empty
//   }

//   pub fn system(config: &'a SystemConfig) -> SystemContext<'a> {
//     SystemContext::new(config)
//   }

//   pub fn push_device(&'a self, device: &'a Device) -> DeviceContext<'a> {
//     DeviceContext { device, _parent: Some(self), _qpath: self.qpath().append(&device.id), }
//   }

//   pub fn push_peripheral(&'a self, peripheral: &'a Peripheral) -> PeripheralContext<'a> {
//     PeripheralContext { peripheral, _parent: Some(self), _qpath: self.qpath().append(&peripheral.id), }
//   }

//   pub fn as_device_context(&self) -> anyhow::Result<&DeviceContext<'a>> {
//     match self {
//       GenerationContext::Device(x) => Ok(x),
//       _ => anyhow::bail!("Not a device!")
//     }
//   }

//   pub fn as_peripheral_context(&self) -> anyhow::Result<&PeripheralContext<'a>> {
//     match self {
//       GenerationContext::Peripheral(x) => Ok(x),
//       _ => anyhow::bail!("Not a peripheral!")
//     }
//   }

//   pub fn find_device(&'a self) -> anyhow::Result<&DeviceContext<'a>> {
//     match self {
//       GenerationContext::Device(x) => Ok(x),
//       _ => match self.parent() {
//         Some(parent) => parent.find_device(),
//         None => anyhow::bail!("Could not find device context"),
//       }
//     }
//   }


// //   fn id(&self) -> &str {
// //     match self {
// //       GenerationContext::System(x) => x.id(),
// //       GenerationContext::Device(x) => x.id(),
// //       GenerationContext::Peripheral(x) => x.id(),
// //     }
// //   }

//   fn parent(&self) -> Option<&GenerationContext> {
//     match self {
//       Self::Empty => None,
//       Self::System(x) => x.parent(),
//       Self::Device(x) => x.parent(),
//       Self::Peripheral(x) => x.parent(),
//   }
// }

//   fn qpath(&self) -> &QualifiedPath {
//     match self {
//       Self::Empty => QualifiedPath::empty_ref(),
//       Self::System(x) => x.qpath(),
//       Self::Device(x) => x.qpath(),
//       Self::Peripheral(x) => x.qpath(),
//     }
//   }

// }