pub mod device;
pub mod block_instance;
pub mod circuit;
pub mod task;
pub mod system;
pub mod block_def;

use serde::{Serialize, Deserialize};
use strum::EnumString;

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
      FieldValue::Float(x) => x.to_string(),
      FieldValue::String(x) => x.to_string()
    }
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumString)]
pub enum FieldType {
  Int,
  Float,
  Bool,
  String,
}
