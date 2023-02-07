use crate::dsl::task::{TaskConfigEnum, FixedStepTaskConfig};
use crate::util::context::GenerationContext;
use super::traits::CodeGenerator;
use genco::prelude::{rust, quote};
use lazy_static::lazy_static;

#[allow(non_snake_case)]
pub struct Imports {
  pub OnceCell: rust::Import,
}

lazy_static! {
  pub static ref IMPORTS: Imports = Imports {
    OnceCell: rust::import("flowmbed_dynsys::util::containers", "OnceCell"),
  };
}


pub struct TaskGenerator<'a> {
  pub task: &'a TaskConfigEnum,
}

impl<'a> TaskGenerator<'a> {
  pub fn new(task: &'a TaskConfigEnum) -> TaskGenerator<'a> {
    TaskGenerator {task}
  }
}

impl<'a> CodeGenerator for TaskGenerator<'a> {  
  fn generate(&self, context: &dyn GenerationContext) -> anyhow::Result<rust::Tokens> {
    match self.task {
      TaskConfigEnum::FixedStepTask(x) => 
        FixedStepTaskGenerator::new(x).generate(context)
    }
  }
}

pub struct FixedStepTaskGenerator<'a> {
  pub task: &'a FixedStepTaskConfig,
  pub ds_core: rust::Import,
}

impl<'a> FixedStepTaskGenerator<'a> {
  pub fn new(task: &'a FixedStepTaskConfig) -> FixedStepTaskGenerator<'a> {
    FixedStepTaskGenerator {
      task,
      ds_core: rust::import("flowmbed_dynsys", "core").with_alias("ds_core"),
    }
  }
}

impl<'a> CodeGenerator for FixedStepTaskGenerator<'a> {
  fn generate(&self, _: &dyn GenerationContext) -> anyhow::Result<genco::prelude::rust::Tokens> {
      let task_name = &self.task.id;
      let ciruit_type = &self.task.circuit;

      Ok(quote!(
        struct $(task_name) {}

        // PeripheralsStruct<'a>
        impl $(task_name) {
          pub fn run<'a>() -> anyhow::Result<()> {
            use $(&self.ds_core)::SystemRunner;

            // Determine the type of the peripheral structure
            type PeripheralsStruct<'a> = <$(ciruit_type)<'a> as RequirePeripherals>::PeripheralsStruct;
            let storage = $(&self.ds_core)::HeapSystemStorage::new($(ciruit_type)::SIZE);
            let mut peripherals: PeripheralsStruct = PeripheralsStruct::DEFAULT;
            peripherals.init()?;

            // static mut CIRCUIT: $(&IMPORTS.OnceCell)<$(circuit_name)<'static>> =  
            
            let mut circuit: $(&IMPORTS.OnceCell)<$(ciruit_type)> = $(&IMPORTS.OnceCell)::DEFAULT;
            $(ciruit_type)::init(&mut circuit, &storage, &mut peripherals)?;

            let run_settings = $(&self.ds_core)::FixedStepRunSettings {
              t_step: $(format!("{:.6}", self.task.t_step)), 
              speedup: $(format!("{:.6}", self.task.speedup)), 
              t_end: None,
              ..Default::default()
            };
            let mut runner = $(&self.ds_core)::FixedStepRunner::new(run_settings);

            runner.init(circuit.get_mut()?)?;
            runner.run(circuit.get_mut()?)?;

            Ok(())
          }
        }
      ))
  }
}


