use std::env;
use simple_logger;

use flowmbed_declarative::gen::system::SystemGenerator;

fn main() -> anyhow::Result<()> {
  simple_logger::SimpleLogger::new().env().init().unwrap();

  let args: Vec<String> = env::args().collect();
  let yaml_input = &args[1];
  let rust_output = &args[2];
  println!("Generating rust code");
  println!("{} -> {}", yaml_input, rust_output);
  

  let generator  = SystemGenerator::from_yaml(&yaml_input)?;
  generator.generate_file(rust_output)?;

  Ok(())
}