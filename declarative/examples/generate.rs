extern crate flowmbed_declarative;
use flowmbed_declarative::gen::system::SystemGenerator;
use std::env;

fn main() -> anyhow::Result<()> {
  let args: Vec<String> = env::args().collect();
  let yaml_input = &args[1];
  let rust_output = &args[2];
  println!("Generating rust code");
  println!("{} -> {}", yaml_input, rust_output);
  

  let generator  = SystemGenerator::from_yaml(&yaml_input)?;
  // let output = generator.generate();
  // println!("\n\n================================\n\n");
  // println!("{}", output.to_file_string()?);
  generator.generate_file(rust_output)?;

  Ok(())
}