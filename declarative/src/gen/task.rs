use crate::dsl::task::{TaskConfigEnum, FixedStepTaskConfig};
use super::traits::CodeGenerator;
use genco::prelude::{rust, quote};

pub struct TaskGenerator<'a> {
  pub task: &'a TaskConfigEnum,
}

impl<'a> TaskGenerator<'a> {
  pub fn new(task: &'a TaskConfigEnum) -> TaskGenerator<'a> {
    TaskGenerator {task}
  }
}

impl<'a> CodeGenerator for TaskGenerator<'a> {  
  fn generate(&self) -> anyhow::Result<rust::Tokens> {
    match self.task {
      TaskConfigEnum::FixedStepTask(x) => 
        FixedStepTaskGenerator::new(x).generate()
    }
  }
}

pub struct FixedStepTaskGenerator<'a> {
  pub task: &'a FixedStepTaskConfig,
  pub fds_core: rust::Import,
}

impl<'a> FixedStepTaskGenerator<'a> {
  pub fn new(task: &'a FixedStepTaskConfig) -> FixedStepTaskGenerator<'a> {
    FixedStepTaskGenerator {
      task,
      fds_core: rust::import("flowmbed_dynsys", "core").with_alias("fds_core"),
    }
  }
}

impl<'a> CodeGenerator for FixedStepTaskGenerator<'a> {
  fn generate(&self) -> anyhow::Result<genco::prelude::rust::Tokens> {
      let task_name = &self.task.id;
      let ciruit_type = &self.task.circuit;

      Ok(quote!(
        struct $(task_name) {}

        impl $(task_name) {
          pub fn run() -> anyhow::Result<()> {
            use $(&self.fds_core)::SystemRunner;

            type PeripheralsStruct<'a> = <$(ciruit_type)<'a> as RequirePeripherals>::PeripheralsStruct;
            let storage = $(&self.fds_core)::HeapSystemStorage::new($(ciruit_type)::SIZE);
            let mut peripherals = PeripheralsStruct::new();
            let mut circuit = $(ciruit_type)::new(&storage, &mut peripherals)?;

            let run_settings = $(&self.fds_core)::FixedStepRunSettings {
              t_step: $(format!("{:.6}", self.task.t_step)), 
              speedup: $(format!("{:.6}", self.task.speedup)), 
              t_end: None,
              ..Default::default()
            };
            let mut runner = $(&self.fds_core)::FixedStepRunner::new(&mut circuit, run_settings);

            runner.init()?;
            runner.run()?;

            Ok(())
          }
        }
      ))
  }
}


