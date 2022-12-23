use std::env;
use simple_logger;
use flowmbed_declarative::gen::{block_def::BlockDefinitionGenerator, self};

fn main() -> anyhow::Result<()> {
  simple_logger::SimpleLogger::new().env().init().unwrap();

  let args: Vec<String> = env::args().collect();
  let yaml_input = &args[1];
  let rust_output = &args[2];

  let generator = BlockDefinitionGenerator::from_yaml(&yaml_input)?;
  generator.generate_files(rust_output)?;

  Ok(())
}