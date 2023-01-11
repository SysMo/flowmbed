use std::env;
use simple_logger;
use log::*;

use flowmbed_declarative::gen::system::SystemGenerator;

fn main() -> anyhow::Result<()> {
  simple_logger::SimpleLogger::new().env().init().unwrap();

  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];
  let yaml_input = &format!("{}.yaml", file_path);
  let rust_output = &format!("{}_gen.rs", file_path);
  info!("Generating rust code");
  info!("{} -> {}", yaml_input, rust_output);
  

  let generator  = SystemGenerator::from_yaml(&yaml_input)?;
  generator.generate_file(rust_output)?;

  Ok(())
}