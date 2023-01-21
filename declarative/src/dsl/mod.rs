pub mod references;
pub mod device;
pub mod block_instance;
pub mod circuit;
pub mod task;
pub mod system;
pub mod block_def;
pub mod rust;

use serde::{Serialize, Deserialize};
use strum::{EnumString, Display};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FieldValue {
  Int(i64),
  Float(f64),
  Bool(bool),
  String(String),
}

impl FieldValue {
  pub fn as_text(&self) -> String {
    match self {
      FieldValue::Int(x) => x.to_string(),
      FieldValue::Bool(x) => x.to_string(),
      FieldValue::Float(x) => format!("{:e}", x),
      FieldValue::String(x) => x.to_string()
    }
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumString, Display)]
pub enum FieldType {
  Int,
  Float,
  Bool,
  String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumString, Display)]
pub enum FieldKind {
  Parameter,
  Input,
  Output,
  DiscreteState,
  ContinuousState
}

#[allow(dead_code)]
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct StorageSize {
    pub r_param: usize,
    pub b_param: usize,
    pub i_param: usize,

    pub r_dstate: usize,
    pub b_dstate: usize,
    pub i_dstate: usize,
    
    pub r_out: usize,
    pub b_out: usize,
    pub i_out: usize
}
