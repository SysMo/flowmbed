use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TaskConfigEnum {
  FixedStepTask(FixedStepTaskConfig)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FixedStepTaskConfig {
  pub id: String,
  pub device: String,
  pub circuit: String,
  pub t_step: f64,
  pub speedup: f64,
}
