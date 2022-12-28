use std::fs;
use crate::dsl::{block_def::BlockDefinition, FieldType, FieldValue, FieldKind, StorageSize};
use genco::prelude::{rust, quote};
use super::traits::CodeGenerator;
use log::*;

/// Generate code for block definition
pub struct BlockDefinitionGenerator {
  block_def: BlockDefinition
}

impl BlockDefinitionGenerator {
  pub fn from_yaml(file_path: &str) -> anyhow::Result<BlockDefinitionGenerator> {

    let yaml = match fs::read_to_string(file_path) {
      Ok(x) => x,
      Err(e) => anyhow::bail!(e)
    };

    let block_def: BlockDefinition = match serde_yaml::from_str(&yaml) {
      Ok(x) => x,
      Err(e) => anyhow::bail!(e)
    };

    println!("{:#?}", block_def);

    Ok(BlockDefinitionGenerator {
      block_def
    })

  }

  pub fn generate_files(&self, output_path: &str) -> anyhow::Result<()> {
    let decl_gen = BlockDeclGenerator::new(&self.block_def);
    let tokens = decl_gen.generate()?;
    let text = tokens.to_file_string()?;
    let output_file = &format!("{}_decl.rs", output_path);
    info!("Generating declaration file {}", output_file);
    fs::write(output_file, text)?;

    let impl_gen = BlockImplGenerator::new(&self.block_def);
    let tokens = impl_gen.generate()?;
    let text = tokens.to_file_string()?;
    let output_file = &format!("{}_impl.rs", output_path);
    info!("Generating implementation file {}", output_file);
    fs::write(output_file, text)?;

    Ok(())
  }
}


// ==============================
pub struct BlockDeclGenerator<'a> {
  block_def: &'a BlockDefinition,
  dynsys_core: rust::Import,
}

impl<'a> BlockDeclGenerator<'a> {
  pub fn new(block_def: &'a BlockDefinition) -> BlockDeclGenerator<'a> {
    BlockDeclGenerator {
      block_def,
      dynsys_core: rust::import("flowmbed_dynsys", "core"),
    }
  }

  fn declare_block(&self) -> anyhow::Result<rust::Tokens> {
    let ds_core = &self.dynsys_core;
    Ok(quote!(
      /// Declare the block struct
      pub struct $(&self.block_def.name)<'a> {
        $(for field in &self.block_def.parameters join (,$['\r']) => 
          $(self.decl_field(&field.name, &FieldKind::Parameter, &field.tpe))
        ),

        $(for field in &self.block_def.inputs join (,$['\r']) => 
          $(self.decl_field(&field.name, &FieldKind::Input, &field.tpe))
        ),

        $(for field in &self.block_def.outputs join (,$['\r']) => 
          $(self.decl_field(&field.name, &FieldKind::Output, &field.tpe))
        ),

        $(for field in &self.block_def.discrete_states join (,$['\r']) => 
          $(self.decl_field(&field.name, &FieldKind::DiscreteState, &field.tpe))
        )
      }
    ))

  }

  fn decl_field(&self, name: &str, kind: &FieldKind, tpe: &FieldType) -> rust::Tokens {
    // let method = if (kind == "")
    quote!(
      $(name): $(&self.dynsys_core)::$(kind.to_string())<'a, $(&self.dynsys_core)::$(tpe.to_string())>
    )
  }


  fn implement_block(&self) -> anyhow::Result<rust::Tokens> {
    let ds_core = &self.dynsys_core;
    Ok(quote!(
      /// Implement the block struct
      impl<'a>  $(&self.block_def.name)<'a> {
        pub fn new<ST: $(ds_core)::DefaultSystemStrorage>(
          builder: &mut $(ds_core)::SystemStorageBuilder<'a, ST>
        ) -> $(&self.block_def.name)<'a> {
          $(&self.block_def.name) {
            $(for field in &self.block_def.parameters join (,$['\r']) => 
              $(self.init_field(&field.name, &FieldKind::Parameter, &field.default))
            ),
    
            $(for field in &self.block_def.inputs join (,$['\r']) => 
              $(self.init_input(&field.name))
            ),
    
            $(for field in &self.block_def.outputs join (,$['\r']) => 
              $(self.init_field(&field.name, &FieldKind::Output, &field.default))
            ),
    
            $(for field in &self.block_def.discrete_states join (,$['\r']) => 
              $(self.init_field(&field.name, &FieldKind::DiscreteState, &field.initial))
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

      impl<'a> Builder<'a> {
        $(for field in &self.block_def.parameters join ($['\r']) => 
          $(self.create_field_setter(&field.name, &field.tpe))
        )
  
      }

      impl<'a> From<BlockBuilder<'a>> for $(&self.block_def.name)<'a> {
        fn from(x: Builder<'a>) -> Self {
          x.component
        }
      }   
    ))
  }

  fn create_field_setter(&self, name: &str, tpe: &FieldType) -> rust::Tokens {
    quote!(
      pub fn $(name)(mut self, v: $(tpe.to_string())) -> Self {
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

impl<'a> CodeGenerator for BlockDeclGenerator<'a> {
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
  dynsys_core: rust::Import,
}

impl<'a> BlockImplGenerator<'a> {
  pub fn new(block_def: &'a BlockDefinition) -> BlockImplGenerator<'a> {
    BlockImplGenerator {
      block_def,
      dynsys_core: rust::import("flowmbed_dynsys", "core"),
    }
  }
}

impl<'a> CodeGenerator for BlockImplGenerator<'a> {
  fn generate(&self) -> anyhow::Result<rust::Tokens> {
    let ds_core = &self.dynsys_core;

    Ok(quote!(
      /// Implementation
      impl<'a> $(ds_core)::DynamicalSystem for $(&self.block_def.name)<'a> {
        fn init(&mut self) -> anyhow::Result<()> {
          Ok(())
        }

        fn step(&mut self, ssi: &$(ds_core)::SystemStateInfo) -> anyhow::Result<()> {

          Ok(())
        }
      }
    ))
  }
}

