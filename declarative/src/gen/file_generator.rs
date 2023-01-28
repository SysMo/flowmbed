use std::path;
use std::fs;
use log::*;
use crate::util::GenerationContext;

use super::traits::CodeGenerator;
use genco::fmt;
use genco::prelude::{rust, Rust};

pub struct FileGenerator<'a, P: AsRef<path::Path>> {
  base_path: P,
  file_name: &'a str,
  gen: &'a dyn CodeGenerator,
  overwrite: bool,
}

impl<'a, P: AsRef<path::Path>> FileGenerator<'a, P> {
  pub fn new(base_path: P, file_name: &'a str, gen: &'a dyn CodeGenerator) -> FileGenerator<'a, P> {
    FileGenerator {
      base_path, file_name, gen, 
      overwrite: true
    }
  }

  pub fn overwrite(mut self, overwrite: bool) -> Self {
    self.overwrite = overwrite;
    self
  }

  pub fn generate(&self) -> anyhow::Result<()> {
    let file_path = self.base_path.as_ref().join(self.file_name);
    if !file_path.exists() || self.overwrite == true {

      info!("Generating file {}", file_path.display());
      let out_file = fs::File::create(file_path)?;
      let mut w = fmt::IoWriter::new(out_file);
    
      let fmt = fmt::Config::from_lang::<Rust>()
          .with_indentation(fmt::Indentation::Space(2));
      let mut formatter = w.as_formatter(&fmt);
      let config = rust::Config::default();
      
      let tokens = self.gen.generate(&GenerationContext::root())?;
      tokens.format_file(&mut formatter, &config)?;


    }
    Ok(())
  }
}