use std::fs;
use std::path;
use genco::prelude::{rust, quote};
use crate::dsl::{block_def::{BlockDefinition, BlockModule}, FieldType, FieldValue, FieldKind, StorageSize};
use super::file_generator::FileGenerator;
use super::traits::CodeGenerator;
use super::comments::Comment;

use log::*;


/// Generate code for block definition
pub struct BlockModuleGenerator<P: AsRef<path::Path>> {
  work_folder: P,
  module_def: BlockModule
}

impl<P: AsRef<path::Path>> BlockModuleGenerator<P> {
  pub fn new(work_folder: P) -> anyhow::Result<BlockModuleGenerator<P>> {
    let mod_yaml_path = work_folder.as_ref().join("mod.yaml").canonicalize()?;

    info!("Reading yaml file: {}", mod_yaml_path.display());
    
    let yaml = match fs::read_to_string(mod_yaml_path) {
      Ok(x) => x,
      Err(e) => anyhow::bail!(e)
    };

    let module_def: BlockModule = match serde_yaml::from_str(&yaml) {
      Ok(x) => x,
      Err(e) => anyhow::bail!(e)
    };

    // println!("{:#?}", module_def);

    Ok(BlockModuleGenerator {
      work_folder,
      module_def
    })

  }

  fn file_name(&self, name: &str, suffix: &str) -> String {
    use convert_case::{Case, Casing};
    let fname = name.from_case(Case::Camel).to_case(Case::Snake);
    format!("{}_{}", fname, suffix)
  }
  
  pub fn generate_files(&self) -> anyhow::Result<()> {

    for block in &self.module_def.blocks {

      FileGenerator::new(
        &self.work_folder, 
        &self.file_name(&block.name, "blck_auto.rs"),
        &BlockAutoGenerator::new(&block)
      ).generate()?;
      

      FileGenerator::new(
        &self.work_folder, 
        &self.file_name(&block.name, "blck_impl.rs"),
        &BlockImplGenerator::new(&block, &self.file_name(&block.name, "blck_auto"))
      ).overwrite(false).generate()?;
    }

    FileGenerator::new(
      &self.work_folder,
      "mod.rs",
      &ModFileGenerator::new(
        self.module_def.blocks
        .iter().map(|x| 
          (self.file_name(&x.name, ""), x.name.clone())
        )
      )
    ).generate()?;

    Ok(())
  }
}


// ==============================
pub struct BlockAutoGenerator<'a> {
  block_def: &'a BlockDefinition,
  dynsys_core: rust::Import,
}

impl<'a> BlockAutoGenerator<'a> {
  pub fn new(block_def: &'a BlockDefinition) -> BlockAutoGenerator<'a> {
    BlockAutoGenerator {
      block_def,
      dynsys_core: rust::import("flowmbed_dynsys", "core").with_alias("dscore"),
    }
  }

  fn declare_block(&self) -> anyhow::Result<rust::Tokens> {
    // let ds_core = &self.dynsys_core;
    Ok(quote!(
      /// Declare the block struct
      #[allow(dead_code)]
      pub struct $(&self.block_def.name)<'a> {
        $(if !self.block_def.parameters.is_empty() =>
          $(for field in &self.block_def.parameters join (,$['\r']) => 
            $(self.decl_field(&field.name, &FieldKind::Parameter, &field.tpe))
          ),
        )

        $(if !self.block_def.inputs.is_empty() =>
          $(for field in &self.block_def.inputs join (,$['\r']) => 
            $(self.decl_field(&field.name, &FieldKind::Input, &field.tpe))
          ),
        )

        $(if !self.block_def.outputs.is_empty() =>
          $(for field in &self.block_def.outputs join (,$['\r']) => 
            $(self.decl_field(&field.name, &FieldKind::Output, &field.tpe))
          ),
        )

        $(if !self.block_def.discrete_states.is_empty() =>
          $(for field in &self.block_def.discrete_states join (,$['\r']) => 
            $(self.decl_field(&field.name, &FieldKind::DiscreteState, &field.tpe))
          ),
        )
      }
    ))

  }

  fn decl_field(&self, name: &str, kind: &FieldKind, tpe: &FieldType) -> rust::Tokens {
    // let method = if (kind == "")
    quote!(
      pub $(name): $(&self.dynsys_core)::$(kind.to_string())<'a, $(&self.dynsys_core)::$(tpe.to_string())>
    )
  }


  fn implement_block(&self) -> anyhow::Result<rust::Tokens> {
    let ds_core = &self.dynsys_core;
    Ok(quote!(
      /// Implement the block struct
      #[allow(dead_code)]
      impl<'a>  $(&self.block_def.name)<'a> {
        pub fn new<ST: $(ds_core)::DefaultSystemStrorage>(
          builder: &mut $(ds_core)::SystemStorageBuilder<'a, ST>
        ) -> $(&self.block_def.name)<'a> {
          $(&self.block_def.name) {
            $(if !self.block_def.parameters.is_empty() =>
              $(for field in &self.block_def.parameters join (,$['\r']) => 
                $(self.init_field(&field.name, &FieldKind::Parameter, &field.default))
              ),
            )
    
            $(if !self.block_def.inputs.is_empty() =>
              $(for field in &self.block_def.inputs join (,$['\r']) => 
                $(self.init_input(&field.name))
              ),
            )
            $(if !self.block_def.outputs.is_empty() =>    
              $(for field in &self.block_def.outputs join (,$['\r']) => 
                $(self.init_field(&field.name, &FieldKind::Output, &field.default))
              ),
            )
      
            $(if !self.block_def.discrete_states.is_empty() =>
              $(for field in &self.block_def.discrete_states join (,$['\r']) => 
                $(self.init_field(&field.name, &FieldKind::DiscreteState, &field.initial))
              )
            )
          }
        }

        pub fn builder<ST: $(ds_core)::DefaultSystemStrorage>(
          builder: &mut $(ds_core)::SystemStorageBuilder<'a, ST>
        ) -> BlockBuilder<'a> {
          BlockBuilder { component: Self::new(builder) }
        }
      }
    ))

  }

  fn init_field(&self, name: &str, kind: &FieldKind, value: &FieldValue) -> rust::Tokens {
    let method = match kind {
      FieldKind::Parameter => "create_param",
      // Not used
      FieldKind::Input => "create_input",
      FieldKind::Output => "create_output",
      FieldKind::DiscreteState => "create_discrete_state",
      FieldKind::ContinuousState => "create_continuous_state",
    };
    quote!(
      $(name): builder.$(method)($(value.as_text()))
    )
  }

  fn init_input(&self, name: &str) -> rust::Tokens {
    quote!(
      $(name): builder.create_input()
    )
  }


  fn create_builder(&self) -> anyhow::Result<rust::Tokens> {
    Ok(quote!(
      pub struct BlockBuilder<'a> {
        component: $(&self.block_def.name)<'a>
      }

      #[allow(dead_code)]
      impl<'a> BlockBuilder<'a> {
        $(for field in &self.block_def.parameters join ($['\r']) => 
          $(self.create_field_setter(&field.name, &field.tpe))
        )
  
      }

      impl<'a> From<BlockBuilder<'a>> for $(&self.block_def.name)<'a> {
        fn from(x: BlockBuilder<'a>) -> Self {
          x.component
        }
      }   
    ))
  }

  fn create_field_setter(&self, name: &str, tpe: &FieldType) -> rust::Tokens {
    quote!(
      pub fn $(name)(mut self, v: $(&self.dynsys_core)::$(tpe.to_string())) -> Self {
        self.component.$(name).reset(v);
        self
      }
    )
  }

  fn impl_has_storage(&self) -> anyhow::Result<rust::Tokens> {
    let ds_core = &self.dynsys_core;
    let mut ss = StorageSize::default();
    
    for field in &self.block_def.parameters {
      match field.tpe {
        FieldType::Int => ss.i_param += 1,
        FieldType::Float => ss.r_param += 1,
        FieldType::Bool => ss.b_param += 1,
        FieldType::String => todo!(),
      }
    }

    for field in &self.block_def.outputs {
      match field.tpe {
        FieldType::Int => ss.i_out += 1,
        FieldType::Float => ss.r_out += 1,
        FieldType::Bool => ss.b_out += 1,
        FieldType::String => todo!(),
      }
    }

    for field in &self.block_def.discrete_states {
      match field.tpe {
        FieldType::Int => ss.i_dstate += 1,
        FieldType::Float => ss.r_dstate += 1,
        FieldType::Bool => ss.b_dstate += 1,
        FieldType::String => todo!(),
      }
    }

    Ok(quote!(
      impl<'a> $(ds_core)::RequiresStorage for $(&self.block_def.name)<'a> {
        const SIZE: $(ds_core)::StorageSize = $(ds_core)::StorageSize {
          r_param: $(ss.r_param), b_param: $(ss.b_param), i_param: $(ss.i_param),      
          r_out: $(ss.r_out), b_out: $(ss.b_out), i_out: $(ss.i_out),
          r_dstate: $(ss.r_dstate), b_dstate: $(ss.b_dstate), i_dstate: $(ss.i_dstate),          
        };
      }
    ))
  }
}

impl<'a> CodeGenerator for BlockAutoGenerator<'a> {
  fn generate(&self) -> anyhow::Result<rust::Tokens> {
    
    Ok(quote!(
      $(self.declare_block()?)

      $(self.implement_block()?)

      $(self.create_builder()?)

      $(self.impl_has_storage()?)
    ))
  }
}

/// ==============================
/// BlockImplGenerator
/// ==============================
pub struct BlockImplGenerator<'a> {
  block_def: &'a BlockDefinition,
  blck_auto_name: &'a str,
  dynsys_core: rust::Import,
}

impl<'a> BlockImplGenerator<'a> {
  pub fn new(block_def: &'a BlockDefinition, blck_auto_name: &'a str ) -> BlockImplGenerator<'a> {
    BlockImplGenerator {
      block_def,
      blck_auto_name,
      dynsys_core: rust::import("flowmbed_dynsys", "core").with_alias("dscore"),
    }
  }
}

impl<'a> CodeGenerator for BlockImplGenerator<'a> {
  fn generate(&self) -> anyhow::Result<rust::Tokens> {
    let ds_core = &self.dynsys_core;

    Ok(quote!(
      use super::$(self.blck_auto_name)::*;
      
      /// Implementation
      #[allow(unused_variables)]
      impl<'a> $(ds_core)::DynamicalSystem for $(&self.block_def.name)<'a> {
        fn init(&mut self) -> anyhow::Result<()> {
          $(Comment([">>> Begin section @DynamicalSystem::init"]))
          Ok(())
          $(Comment([">>> End section @DynamicalSystem::init"]))
        }

        fn step(&mut self, ssi: &$(ds_core)::SystemStateInfo) -> anyhow::Result<()> {
          $(Comment([">>> Begin section @DynamicalSystem::step"]))
          Ok(())
          $(Comment([">>> End section @DynamicalSystem::step"]))
        }
      }
    ))
  }
}

/// ==============================
/// ModFileGenerator
/// ==============================
pub struct ModFileGenerator {
  block_files: Vec<(String, String)>,
  dynsys_core: rust::Import,
}

impl ModFileGenerator {
  pub fn new<C: Iterator<Item = (String, String)>>(block_files: C) -> ModFileGenerator {
    ModFileGenerator {
      block_files: block_files.collect(),
      dynsys_core: rust::import("flowmbed_dynsys", "core").with_alias("dscore"),
    }
  }
}

impl CodeGenerator for ModFileGenerator {
  fn generate(&self) -> anyhow::Result<rust::Tokens> {
    // let ds_core = &self.dynsys_core;

    Ok(quote!(
      $(for (file, block) in &self.block_files => 
        mod $(file)blck_auto;
        mod $(file)blck_impl;

        pub use $(file)blck_auto::$(block);
      )
    ))
  }
}