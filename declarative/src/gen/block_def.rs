use std::fs;
use crate::dsl::block_def::BlockDefinition;
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
  block_def: &'a BlockDefinition
}

impl<'a> BlockDeclGenerator<'a> {
  pub fn new(block_def: &'a BlockDefinition) -> BlockDeclGenerator<'a> {
    BlockDeclGenerator {block_def}
  }
}

impl<'a> CodeGenerator for BlockDeclGenerator<'a> {
  fn generate(&self) -> anyhow::Result<rust::Tokens> {
    Ok(quote!(
      /// Declarations
    ))
  }
}


// ==============================
pub struct BlockImplGenerator<'a> {
  block_def: &'a BlockDefinition
}

impl<'a> BlockImplGenerator<'a> {
  pub fn new(block_def: &'a BlockDefinition) -> BlockImplGenerator<'a> {
    BlockImplGenerator {block_def}
  }
}

impl<'a> CodeGenerator for BlockImplGenerator<'a> {
  fn generate(&self) -> anyhow::Result<rust::Tokens> {
    Ok(quote!(
      /// Implementation
    ))
  }
}

