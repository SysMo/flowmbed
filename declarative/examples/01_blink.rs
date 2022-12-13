extern crate flowmbed_declarative;
use serde::{Serialize, Deserialize};
use std::fs;
use flowmbed_declarative::dsl::board::BoardConfig;

fn main() -> Result<(), serde_yaml::Error> {
  let file_path = "examples/01_blink.yaml";
  let yaml = fs::read_to_string(file_path)
    .expect("Cannot read the file");
  

  let config: BoardConfig = serde_yaml::from_str(&yaml)?;
  println!("{:#?}", config);
  Ok(())
}