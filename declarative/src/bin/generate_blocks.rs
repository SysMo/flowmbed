use std::env;
use std::path;
use simple_logger;
use flowmbed_declarative::gen::{block_def::BlockModuleGenerator};
use log::*;
use glob::glob;

fn main() -> anyhow::Result<()> {
  simple_logger::SimpleLogger::new().env().init().unwrap();

  let args: Vec<String> = env::args().collect();
  let work_folder = path::Path::new(&args[1]);
  // info!("Running in: {}", env::current_dir()?.display());

  let pattern = work_folder.join("*/mod.yaml").into_os_string().into_string().unwrap();
  info!("Searching for block definitions in {}", pattern);

  for entry in glob(&pattern)? {
    match entry {
      Ok(mut path) => {
        info!("Found block definition file {}", path.display());
        path.pop();
        let generator = BlockModuleGenerator::new(path)?;
        generator.generate_files()?;
      }
      Err(e) => println!("{:?}", e),      
    }
  }

  Ok(())
}