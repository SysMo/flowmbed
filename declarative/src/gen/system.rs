use std::fs;
use crate::dsl::system::SystemConfig;
use crate::util::context::{GenerationContext, system_context};
use genco::prelude::{rust, quote};
use super::resolver::{NameResolver, NameResolverImpl};
use super::traits::CodeGenerator;
use super::device::DeviceGenerator;
use super::circuit::CircuitGenerator;
use super::task::TaskGenerator;
use super::comments::Comment;

/// Generates code for the MCU system
pub struct SystemGenerator {
  pub system: SystemConfig
}

impl SystemGenerator {
  pub fn from_yaml(file_path: &str) -> anyhow::Result<SystemGenerator> {

    let yaml = match fs::read_to_string(file_path) {
      Ok(x) => x,
      Err(e) => anyhow::bail!(e)
    };

    let system: SystemConfig = match serde_yaml::from_str(&yaml) {
      Ok(x) => x,
      Err(e) => anyhow::bail!(e)
    };

    // println!("{:#?}", system);

    Ok(SystemGenerator {
      system
    })

  }

  pub fn generate_file(&self, output_path: &str) -> anyhow::Result<()> {
    let tokens = self.generate(&system_context(&self.system))?;
    let text = tokens.to_file_string()?;
    fs::write(output_path, text)?;
    Ok(())
  }

}

impl CodeGenerator for SystemGenerator {
  fn generate(&self, ctx: &dyn GenerationContext) -> anyhow::Result<rust::Tokens> {
    let name_resolver: &dyn NameResolver = &NameResolverImpl::new(&self.system.imports);

    // let device_gen = self.system.devices.iter()
    //   .map(DeviceGenerator::new);

    let circuit_gen = self.system.circuits.iter()
      .map(|c| CircuitGenerator::new(c, name_resolver));

    let task_gen = self.system.tasks.iter()
    .map(TaskGenerator::new);
    
    let tokens = quote! {
      use flowmbed_core_blocks::cfg_device;
      #[allow(unused_imports)]
      use flowmbed_dynsys::core::{Float, Int, Bool, String};
      
      $(for device in &self.system.devices =>        
        $(DeviceGenerator::new(
          &device,
          ctx.as_system_context()?.push_device(&device)
        ).generate()?)$['\n']
      )

      $(for gen in circuit_gen =>        
        $(gen.generate(ctx)?)$['\n']
      )

      $(for gen in task_gen =>        
        $(gen.generate(ctx)?)$['\n']
      )

      fn main() -> anyhow::Result<()> {
        $(Comment("Configure logging"))
        cfg_device::config_logger();
        $(Comment("Start the main task"))
        MainTask::run()?;

        Ok(())
      }
    
    };
    Ok(tokens)
  }

}