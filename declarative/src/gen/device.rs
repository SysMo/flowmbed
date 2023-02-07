use crate::dsl::device::{
  Device, DeviceConfig, Peripheral, PeripheralConfig,
}; 
use crate::util::context::{GenerationContext, DeviceContext, PeripheralContext};
use genco::prelude::{rust, quote};
#[allow(unused_imports)]
use super::comments::{Comment, DocComment};
use convert_case::{Case, Casing};
use lazy_static::lazy_static;

lazy_static! {
  pub static ref REF_ONCE: rust::Import = 
    rust::import("flowmbed_dynsys::util::containers","RefOnce");
}
pub struct DeviceGenerator<'a> {
  pub config: &'a dyn DeviceConfig,
  pub context: DeviceContext<'a>,
}

impl<'a> DeviceGenerator<'a> {
  pub fn new(device: &'a Device, context: DeviceContext<'a>) -> Self {    
    DeviceGenerator {
      config: device.config.as_ref(),
      context
    }    
  }

  fn gen_imports(&self) -> anyhow::Result<rust::Tokens> {
    (self.config as &dyn DeviceConfigGenerator).gen_imports(&self.context)
  }

  fn gen_take_peripherals(&self) -> anyhow::Result<rust::Tokens> {
    (self.config as &dyn DeviceConfigGenerator).gen_take_peripherals(&self.context)
  }

  fn decl_peripheral(&self, pg: PeripheralGenerator) -> anyhow::Result<rust::Tokens> {
    let peripheral_id = pg.context.long_id();
    Ok(quote!(
      $(for child in pg.child_peripherals() =>
        $(self.decl_peripheral(PeripheralGenerator::new(
          &child, pg.context.push_peripheral(&child)
        ))?)
      )
      $(peripheral_id): $(&*REF_ONCE)<$(pg.gen_type()?)>,$['\r']
    ))
  }

  fn initialize_peripheral(&self, pg: PeripheralGenerator) -> anyhow::Result<rust::Tokens> {
    let device_context = &self.context;
    let var_device_peripherals = device_context.var_device_periph();
    let peripheral_id = &pg.context.long_id();

    Ok(quote!(
      $(for child in pg.child_peripherals() =>
        $(self.initialize_peripheral(PeripheralGenerator::new(
          &child, pg.context.push_peripheral(&child)
        ))?)
      )
      $(var_device_peripherals).$(peripheral_id).init({
        $(pg.gen_initialize()?)
      })?;$['\r']
      println!("Initialized peripheral {}", stringify!($(peripheral_id)));
    ))
  }

  pub fn generate(&self) -> anyhow::Result<rust::Tokens> {
    let device_name = &self.context.id();

    let peripherals_struct = &format!("{}Peripherals",
      device_name.from_case(Case::Snake).to_case(Case::UpperCamel));

    let peripherals = self.config.peripherals();
    let device_context = &self.context;

    Ok(quote!(
      $(self.gen_imports()?)

      #[derive(const_default_derive::ConstDefault)]
      struct $(peripherals_struct)<'a> {
        __marker: std::marker::PhantomData<&'a ()>,
        __pin: std::marker::PhantomPinned,
        $(for peripheral in peripherals =>
          $(self.decl_peripheral(PeripheralGenerator::new(
            &peripheral,
            self.context.push_peripheral(&peripheral)
          ))?)
        )  
      }

      impl<'a> $(peripherals_struct)<'a> {
        // pub fn init($(device_context.var_device_periph()): &'a mut $(peripherals_struct)<'a>) -> anyhow::Result<()> {
        pub fn init(&mut self) -> anyhow::Result<()> {
          let $(device_context.var_internal_periph()) = $(self.gen_take_peripherals()?);

          $(for peripheral in peripherals =>            
            $(self.initialize_peripheral(PeripheralGenerator::new(
              &peripheral,
              self.context.push_peripheral(&peripheral)
            ))?)
          )  
          Ok(())
        }
      }
    ))
  }
}


pub struct PeripheralGenerator<'a> {
  pub config: &'a dyn PeripheralConfig,
  pub context: PeripheralContext<'a>,
}

impl<'a> PeripheralGenerator<'a> {
  pub fn new(peripheral: &'a Peripheral, context: PeripheralContext<'a>) -> Self {    
    PeripheralGenerator { 
      config: peripheral.config.as_ref(),
      context
    }    
  }

  fn gen_type(&self) -> anyhow::Result<rust::Tokens> {
    self.config.gen_type(&self.context)
  }

  fn gen_initialize(&self) -> anyhow::Result<rust::Tokens> {
    self.config.gen_initialize(&self.context)
  }

  fn child_peripherals(&self) -> Vec<Peripheral> {
    self.config.child_peripherals(&self.context)
  }
}


pub trait DeviceConfigGenerator {
  fn gen_imports(&self, context: &DeviceContext) -> anyhow::Result<rust::Tokens>;
  fn gen_take_peripherals(&self, context: &DeviceContext) -> anyhow::Result<rust::Tokens>;
}

pub trait PeripheralConfigGenerator {
  fn gen_type(&self, context: &PeripheralContext) -> anyhow::Result<rust::Tokens>;
  fn gen_initialize(&self, context: &PeripheralContext) -> anyhow::Result<rust::Tokens>;
  fn child_peripherals(&self, _context: &PeripheralContext) -> Vec<Peripheral> {
    Vec::new()
  }
}