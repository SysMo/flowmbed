use std::fs;
use std::path;
use genco::prelude::{rust, quote};
use lazy_static::lazy_static;
use crate::dsl::FieldValue;
use crate::dsl::block_def::StructuralDefinition;
use crate::dsl::block_def::StructuralType;
use crate::dsl::rust::{RustTypeRef};
use crate::dsl::{block_def::{BlockDefinition, BlockModule}, FieldType, FieldKind, StorageSize};
use crate::util::context::GenerationContext;
use super::file_generator::FileGenerator;
use super::traits::CodeGenerator;
use super::comments::{Comment, DocComment, BeginSection, EndSection};

use log::*;


/// Generate code for block definition
pub struct BlockModuleGenerator<P: AsRef<path::Path>> {
  work_folder: P,
  module_def: BlockModule
}

impl<P: AsRef<path::Path>> BlockModuleGenerator<P> {
  pub fn new(work_folder: P) -> anyhow::Result<BlockModuleGenerator<P>> {
    let mod_yaml_path = work_folder.as_ref().join("mod.yaml").canonicalize()?;

    // println!("======================================");
    // {
    //   let x1 = FieldType::Generic("T1".to_owned());
    //   let i1 = InputDefinition {
    //     name: "Sink".to_owned(),
    //     tpe: x1,
    //     size: Some(FieldValue::Int(3)),
    //   };
    //   println!("{}", serde_yaml::to_string(&i1)?);
    // }
    // println!("======================================");

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
        &self.file_name(&block.name, "auto.rs"),
        &BlockAutoGenerator::new(&block)
      ).generate()?;
      

      FileGenerator::new(
        &self.work_folder, 
        &self.file_name(&block.name, "impl.rs"),
        &BlockImplGenerator::new(&block, &self.file_name(&block.name, "auto"))
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

#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct Imports {
  pub ds_core: rust::Import
}

lazy_static! {
  pub static ref IMPORTS: Imports = Imports {
    ds_core: rust::import("flowmbed_dynsys", "core").with_alias("ds_core")
  };
}

pub struct BlockGenAux<'a>(&'a BlockDefinition);
impl<'a> BlockGenAux<'a> {
  pub fn get_field_type(&self, tpe: &FieldType) -> rust::Tokens {
    let ds_core = &IMPORTS.ds_core;
    match tpe {
      FieldType::Int => quote!($(ds_core)::Int),
      FieldType::Float => quote!($(ds_core)::Float),
      FieldType::Bool => quote!($(ds_core)::Bool),
      FieldType::String => quote!($(ds_core)::String),
      FieldType::Generic(tpe) => quote!($(tpe))
    }
  }

  fn generic_params(&self, is_decl: bool) -> anyhow::Result<rust::Tokens> {
    let mut gp: Vec<rust::Tokens> = vec![quote!('a)];
    for structural in &self.0.structural {
      match structural {
        StructuralDefinition::Constant(x) => {
          if is_decl {
            gp.push(quote!(const $(&x.name): usize))
          } else {
            gp.push(quote!($(&x.name)))
          }    
        },
        StructuralDefinition::Type(x) => {
          if is_decl && !x.restrictions.is_empty() {            
            gp.push(quote!(
              $(&x.name): $(for r in &x.restrictions join ( + ) => $(r))
            ))
          } else {
            gp.push(quote!($(&x.name)))
          }
        }
          
      }

    }

    Ok(quote!(
      $(for elem in gp join (, ) => $(elem))
    ))
  }
}

// ==============================
pub struct BlockAutoGenerator<'a> {
  block_def: &'a BlockDefinition,
  aux: BlockGenAux<'a>
}

impl<'a> BlockAutoGenerator<'a> {
  pub fn new(block_def: &'a BlockDefinition) -> BlockAutoGenerator<'a> {
    BlockAutoGenerator {
      block_def,
      aux: BlockGenAux(block_def)
    }
  }

  fn structural_types(&self) -> Vec<&StructuralType> {
    self.block_def.structural
      .iter().filter_map(|x| match x {
        StructuralDefinition::Type(x) => Some(x),
        StructuralDefinition::Constant(_) => None,
    }).collect::<Vec<_>>()
  }

  fn declare_block(&self) -> anyhow::Result<rust::Tokens> {
    // let ds_core = &IMPORTS.ds_core;
    Ok(quote!(
      $(DocComment(["Declare the block struct"]))
      #[allow(dead_code)]
      pub struct $(&self.block_def.name)<$(self.aux.generic_params(true)?)> {
        $(if !self.block_def.parameters.is_empty() =>
          $(Comment("Parameters"))
          $(for field in &self.block_def.parameters join (,$['\r']) => 
            $(self.decl_field(&field.name, &FieldKind::Parameter, &field.tpe, &None))
          ),
        )
        $(Comment("Inputs"))
        $(if !self.block_def.inputs.is_empty() =>
          $(for field in &self.block_def.inputs join (,$['\r']) => 
            $(self.decl_io(&field.name, &FieldKind::Input, &field.tpe, &field.size))
          ),
        )
        $(Comment("Outputs"))
        $(if !self.block_def.outputs.is_empty() =>
          $(for field in &self.block_def.outputs join (,$['\r']) => 
            $(self.decl_io(&field.name, &FieldKind::Output, &field.tpe, &field.size))
          ),
        )
        $(Comment("Discrete states"))
        $(if !self.block_def.discrete_states.is_empty() =>
          $(for field in &self.block_def.discrete_states join (,$['\r']) => 
            $(self.decl_field(&field.name, &FieldKind::DiscreteState, &field.tpe, &None))
          ),
        )
        $(Comment("Peripherals"))
        $(if !self.block_def.peripherals.is_empty() =>
          $(for peripheral in &self.block_def.peripherals join (,$['\r']) => 
            $(self.decl_peripheral(peripheral.name(), peripheral.mut_ref(Some("a"))))
          ),
        )
      }
    ))

  }

  fn decl_field(&self, name: &str, kind: &FieldKind, tpe: &FieldType, size: &Option<FieldValue>) -> rust::Tokens {
    let ds_core = &IMPORTS.ds_core;
    let decl_type = match size {
      Some(s) => quote!([$(ds_core)::$(kind.to_string())<'a, $(self.aux.get_field_type(tpe))>; $(s.as_text())]),
      None => quote!($(ds_core)::$(kind.to_string())<'a, $(self.aux.get_field_type(tpe))>)
    };
    quote!(
      pub $(name): $(decl_type)
    )  
  }

  fn decl_io(&self, name: &str, kind: &FieldKind, tpe: &FieldType, size: &Option<FieldValue>) -> rust::Tokens {
    let ds_core = &IMPORTS.ds_core;
    let mut generic_params: Vec<rust::Tokens> = vec![];
    match kind {
        FieldKind::Output => (),
        _ => generic_params.push(quote!('a)),
    };
    let field_type = self.aux.get_field_type(tpe);
    match size {
      Some(s) => generic_params.push(quote!([$(field_type); $(s.as_text())])),
      None => generic_params.push(quote!($(field_type)))
    };
    quote!(
      pub $(name): $(ds_core)::$(kind.to_string())<$(for p in generic_params join (, ) => $(p))>
    )  
  }

  fn decl_peripheral(&self, name: &str, tpe_ref: RustTypeRef) -> rust::Tokens {
    quote!(
      pub $(name): $(tpe_ref)
    )
  }


  fn implement_block(&self) -> anyhow::Result<rust::Tokens> {
    let structural_types = self.structural_types();
    Ok(quote!(
      $(DocComment(["Implement the block struct"]))
      #[allow(dead_code)]
      impl<$(self.aux.generic_params(true)?)>  $(&self.block_def.name)<$(self.aux.generic_params(false)?)> {
        pub fn builder() -> Builder<$(self.aux.generic_params(false)?)> {
          Builder {
            __phantom: std::marker::PhantomData,
            $(for t in structural_types => 
              __phantom_$(&t.name): std::marker::PhantomData,$['\r']
            )
            $(for field in &self.block_def.parameters join ($['\r']) => 
              val_$(&field.name): $(&field.default.as_text()),
            )
            $(if !self.block_def.peripherals.is_empty() =>
              $(for peripheral in &self.block_def.peripherals join ($['\r']) =>
                _$(peripheral.name()): None,
              )
            )              
          }
        }
      }
    ))

  }

  fn decl_builder(&self) -> anyhow::Result<rust::Tokens> {
    let structural_types = self.structural_types();

    Ok(quote!(
      #[allow(non_snake_case)]
      pub struct Builder<$(self.aux.generic_params(true)?)> {
        __phantom: std::marker::PhantomData<&'a ()>,
        $(for t in structural_types => 
          __phantom_$(&t.name): std::marker::PhantomData<$(&t.name)>,$['\r']
        )
        $(if !self.block_def.peripherals.is_empty() =>
          $(for peripheral in &self.block_def.peripherals join ($['\r']) =>
            _$(peripheral.name()): Option<$(peripheral.mut_ref(Some("a")))>,
          )
        )                
        $(for field in &self.block_def.parameters join ($['\r']) => 
          val_$(&field.name): $(self.aux.get_field_type(&field.tpe)),
        )
      }
    ))
  }

  fn impl_builder(&self) -> anyhow::Result<rust::Tokens> {
    // let ds_core = &self.dynsys_core;
    Ok(quote!(
      #[allow(dead_code)]
      impl<$(self.aux.generic_params(true)?)> Builder<$(self.aux.generic_params(false)?)> {
        $(for field in &self.block_def.parameters join ($['\r']) => 
          $(self.create_field_setter(&field.name, &field.tpe))
        )

        $(for peripheral in &self.block_def.peripherals join ($['\r']) =>
          $(self.create_peripheral_setter(peripheral.name(), peripheral.mut_ref(Some("a"))))
        )
      }   
    ))
  }

  fn create_field_setter(&self, name: &str, tpe: &FieldType) -> rust::Tokens {
    quote!(
      pub fn $(name)(mut self, v: $(self.aux.get_field_type(tpe))) -> Self {
        self.val_$(name) = v;
        self
      }
    )
  }

  fn create_peripheral_setter(&self, name: &str, tpe_ref: RustTypeRef) -> rust::Tokens {
    quote!(
      pub fn $(name)(mut self, v: $(tpe_ref)) -> Self {
        self._$(name) = Some(v);
        self
      }
    )
  }


  fn impl_from_builder(&self) -> anyhow::Result<rust::Tokens> {
    let ds_core = &IMPORTS.ds_core;
    let gen_params_use = self.aux.generic_params(false)?;
    Ok(quote!(
      #[allow(unused_variables)]
      impl<$(self.aux.generic_params(true)?)> $(ds_core)::BlockBuilder<'a, $(&self.block_def.name)<$(&gen_params_use)>> for Builder<$(&gen_params_use)> {
        fn build<ST: $(ds_core)::DefaultSystemStrorage>(self, storage_builder: &mut $(ds_core)::SystemStorageBuilder<'a, ST>) -> $(&self.block_def.name)<$(self.aux.generic_params(false)?)> {
          $(&self.block_def.name) {
            $(if !self.block_def.parameters.is_empty() =>
              $(for field in &self.block_def.parameters join (,$['\r']) => 
                $(self.init_field(&field.name, &FieldKind::Parameter, &None, &format!("self.val_{}", field.name)))
              ),
            )
    
            $(if !self.block_def.inputs.is_empty() =>
              $(for field in &self.block_def.inputs join (,$['\r']) => 
                $(self.init_input(&field.name))
              ),
            )

            $(if !self.block_def.outputs.is_empty() =>    
              $(for field in &self.block_def.outputs join (,$['\r']) => 
                $(self.init_output(&field.name, &field.default.as_text()))
              ),
            )
      
            $(if !self.block_def.discrete_states.is_empty() =>
              $(for field in &self.block_def.discrete_states join (,$['\r']) => 
                $(self.init_field(&field.name, &FieldKind::DiscreteState, &None, &field.initial.as_text()))
              ),
            )
            
            $(if !self.block_def.peripherals.is_empty() =>
              $(for peripheral in &self.block_def.peripherals join (,$['\r']) => 
                $(peripheral.name()): self._$(peripheral.name()).unwrap()
              ),
            )

          }
        }
      }         
    ))
  }
      


  fn init_field(&self, name: &str, kind: &FieldKind, size: &Option<FieldValue>, value_source: &str) -> rust::Tokens {
    let method = match (kind, size) {
      (FieldKind::Parameter, None) => "create_param",
      (FieldKind::Parameter, Some(_)) => "create_params",
      (FieldKind::DiscreteState, None) => "create_discrete_state",
      (FieldKind::DiscreteState, Some(_)) => "create_discrete_states",
      (FieldKind::ContinuousState, None) => "create_continuous_state",
      (FieldKind::ContinuousState, Some(_)) => "create_continuous_states",
      _ => ""
    };
    info!("{:?} {:?} {:?} {:?}", name, kind, size, method);
    quote!(
      $(name): storage_builder.$(method)($(value_source))
    )
  }

  fn init_input(&self, name: &str) -> rust::Tokens {
    let ds_core = &IMPORTS.ds_core;
    quote!(
      $(name): $(ds_core)::Input::new()
    )
  }

  fn init_output(&self, name: &str, value_source: &str) -> rust::Tokens {
    let ds_core = &IMPORTS.ds_core;
    quote!(
      $(name): $(ds_core)::Output::new($(value_source))
    )
  }

  fn impl_has_storage(&self) -> anyhow::Result<rust::Tokens> {
    let ds_core = &IMPORTS.ds_core;
    let mut ss = StorageSize::default();
    
    for field in &self.block_def.parameters {
      match field.tpe {
        FieldType::Int => ss.i_param += 1,
        FieldType::Float => ss.r_param += 1,
        FieldType::Bool => ss.b_param += 1,
        FieldType::String => todo!(),
        FieldType::Generic(_) => todo!(),
      }
    }

    // for field in &self.block_def.outputs {
    //   match field.tpe {
    //     FieldType::Int => ss.i_out += 1,
    //     FieldType::Float => ss.r_out += 1,
    //     FieldType::Bool => ss.b_out += 1,
    //     FieldType::String => todo!(),
    //     FieldType::Generic(_) => todo!(),
    //   }
    // }

    for field in &self.block_def.discrete_states {
      match field.tpe {
        FieldType::Int => ss.i_dstate += 1,
        FieldType::Float => ss.r_dstate += 1,
        FieldType::Bool => ss.b_dstate += 1,
        FieldType::String => todo!(),
        FieldType::Generic(_) => todo!(),
      }
    }

    Ok(quote!(
      impl<$(self.aux.generic_params(true)?)> $(ds_core)::RequiresStorage for $(&self.block_def.name)<$(self.aux.generic_params(false)?)> {
        const SIZE: $(ds_core)::StorageSize = $(ds_core)::StorageSize {
          r_param: $(ss.r_param), b_param: $(ss.b_param), i_param: $(ss.i_param),      
          // r_out: $(ss.r_out), b_out: $(ss.b_out), i_out: $(ss.i_out),
          r_dstate: $(ss.r_dstate), b_dstate: $(ss.b_dstate), i_dstate: $(ss.i_dstate),          
        };
      }
    ))
  }

}

impl<'a> CodeGenerator for BlockAutoGenerator<'a> {
  fn generate(&self, _: &dyn GenerationContext) -> anyhow::Result<rust::Tokens> {
    
    Ok(quote!(
      $(self.declare_block()?)

      $(self.implement_block()?)

      $(self.decl_builder()?)

      $(self.impl_builder()?)

      $(self.impl_from_builder()?)

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
  aux: BlockGenAux<'a>
}

impl<'a> BlockImplGenerator<'a> {
  pub fn new(block_def: &'a BlockDefinition, blck_auto_name: &'a str ) -> BlockImplGenerator<'a> {
    BlockImplGenerator {
      block_def,
      blck_auto_name,
      aux: BlockGenAux(block_def)
    }
  }
}

impl<'a> CodeGenerator for BlockImplGenerator<'a> {
  fn generate(&self, _: &dyn GenerationContext) -> anyhow::Result<rust::Tokens> {
    let ds_core = &IMPORTS.ds_core;

    Ok(quote!(
      use super::$(self.blck_auto_name)::*;
      
      $(DocComment(["Implementation DynamicalSystem protocol"]))
      #[allow(unused_variables)]
      impl<$(self.aux.generic_params(true)?)> $(ds_core)::DynamicalSystem<'a> for $(&self.block_def.name)<$(self.aux.generic_params(false)?)> {
        fn init(&mut self) -> anyhow::Result<()> {
          $(BeginSection("DynamicalSystem::init"))
          Ok(())
          $(EndSection("DynamicalSystem::init"))
        }

        fn step(&mut self, ssi: &$(ds_core)::SystemStateInfo) -> anyhow::Result<()> {
          $(BeginSection("DynamicalSystem::step"))
          Ok(())
          $(EndSection("DynamicalSystem::step"))
        }
      }

      $(BeginSection("Begin section @Helpers"))
      struct Aux;
      impl Aux {


      }
      $(EndSection("Begin section @Helpers"))
    ))
  }
}

/// ==============================
/// ModFileGenerator
/// ==============================
pub struct ModFileGenerator {
  block_files: Vec<(String, String)>,
  // dynsys_core: rust::Import,
}

impl ModFileGenerator {
  pub fn new<C: Iterator<Item = (String, String)>>(block_files: C) -> ModFileGenerator {
    ModFileGenerator {
      block_files: block_files.collect(),
    }
  }
}

impl CodeGenerator for ModFileGenerator {
  fn generate(&self, _: &dyn GenerationContext) -> anyhow::Result<rust::Tokens> {
    Ok(quote!(
      $(for (file, block) in &self.block_files join ($['\r']) => 
        mod $(file)auto;
        mod $(file)impl;
        pub use $(file)auto::$(block);$['\r']$['\r']
      )
    ))
  }
}