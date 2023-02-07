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
#[serde(try_from = "String")]
pub enum FieldType {
  Int,
  Float,
  Bool,
  String,
  Generic(String)
}

impl TryFrom<String> for FieldType {
  type Error = anyhow::Error;
  fn try_from(value: String) -> Result<Self, Self::Error> {
    match value.as_str() {
      "Int" => Ok(FieldType::Int),
      "Float" => Ok(FieldType::Float),
      "Bool" => Ok(FieldType::Bool),
      "String" => Ok(FieldType::String),
      t => Ok(FieldType::Generic(t.to_owned()))
    }
  }
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
    
    // pub r_out: usize,
    // pub b_out: usize,
    // pub i_out: usize
}
