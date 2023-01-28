use crate::dsl::device::{
  Device,
}; 
use crate::util::GenerationContext;
use super::traits::CodeGenerator;
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
  pub device: &'a Device,
}

impl<'a> DeviceGenerator<'a> {
  pub fn new(device: &'a Device) -> Self {
    DeviceGenerator { 
      device
    }
  }
}

impl<'a> CodeGenerator for DeviceGenerator<'a> {
  fn generate(&self, context: &GenerationContext) -> anyhow::Result<rust::Tokens> {  
    let device_name = &self.device.id;

    let device_struct = &format!("{}Device",
      device_name.from_case(Case::Snake).to_case(Case::UpperCamel));


    let peripherals_struct = &format!("{}Peripherals",
      device_name.from_case(Case::Snake).to_case(Case::UpperCamel));

    let conf_gen = self.device.gen();

    let peripherals = self.device.config.peripherals();

    fn create_id(ctx: &GenerationContext) -> String {
      // TODO Trace the peripheral path up to the device to ensure unique name
      ctx.id.clone()
    }

    fn decl_peripheral(pg: PeripheralGenerator) -> anyhow::Result<rust::Tokens> {
      Ok(quote!(
        $(for child_gen in pg.child_peripherals() =>
          $(decl_peripheral(child_gen)?)
        )
        $(create_id(&pg.context)): $(&*REF_ONCE)<$(pg.gen_type()?)>,$['\r']
      ))
    }

    fn initialize_peripheral(pg: PeripheralGenerator) -> anyhow::Result<rust::Tokens> {
      Ok(quote!(
        $(for child_gen in pg.child_peripherals() =>
          $(initialize_peripheral(child_gen)?)
        )

        MCU_PERIPHERALS.$(create_id(&pg.context)).init({
          $(pg.gen_initialize()?)
        })?;$['\r']
      ))
    }

    Ok(quote!(
      $(conf_gen.gen_imports(context)?)
      #[derive(const_default_derive::ConstDefault)]
      struct $(peripherals_struct)<'a> {
        __marker: std::marker::PhantomData<&'a ()>,
        $(for peripheral in peripherals =>
          $(decl_peripheral(peripheral.gen(context))?)
        )  
      }

      impl<'a> $(peripherals_struct)<'a> {
        pub fn new() -> anyhow::Result<&'static $(peripherals_struct)<'static>> {
          static MCU_PERIPHERALS: $(peripherals_struct)<'static> = $(peripherals_struct)::DEFAULT;

          let peripherals = $(conf_gen.gen_take_peripherals(&context)?);

          $(for peripheral in peripherals =>
            $(initialize_peripheral(peripheral.gen(context))?)
          )  

          Ok(&MCU_PERIPHERALS)
        }
      }


    ))
  }
}


pub struct PeripheralGenerator<'a> {
  pub context: GenerationContext<'a>,
  pub config: &'a dyn PeripheralConfigGenerator
}

impl<'a> PeripheralGenerator<'a> {
  fn gen_type(&self) -> anyhow::Result<rust::Tokens> {
    self.config.gen_type(&self.context)
  }

  fn gen_initialize(&self) -> anyhow::Result<rust::Tokens> {
    self.config.gen_initialize(&self.context)
  }

  fn child_peripherals(&'a self) -> Box<dyn Iterator<Item = PeripheralGenerator> + 'a> {
    self.config.child_peripherals(&self.context)
  }
}
// pub struct PeripheralGenerator<'a> {
//   pub peripheral: &'a Peripheral,
// }

// impl<'a> PeripheralGenerator<'a> {
//   pub fn new(peripheral: &'a Peripheral) -> Self {
//     PeripheralGenerator { 
//       peripheral
//     }
//   }
// }


pub trait DeviceConfigGenerator {
  fn gen_imports(&self, context: &GenerationContext) -> anyhow::Result<rust::Tokens>;
  fn gen_take_peripherals(&self, context: &GenerationContext) -> anyhow::Result<rust::Tokens>;
}

pub trait PeripheralConfigGenerator {
  fn gen_type(&self, context: &GenerationContext) -> anyhow::Result<rust::Tokens>;
  fn gen_initialize(&self, context: &GenerationContext) -> anyhow::Result<rust::Tokens>;
  fn child_peripherals<'a>(&'a self, context: &'a GenerationContext) -> Box<dyn Iterator<Item = PeripheralGenerator> + 'a> {
    Box::new(core::iter::empty())
  }
}