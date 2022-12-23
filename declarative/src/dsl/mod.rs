pub mod device;
pub mod block_instance;
pub mod circuit;
pub mod task;
pub mod system;
pub mod block_def;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FieldValue {
  Int(i64),
  Float(f64),
  Bool(bool),
  String(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FieldType {
  Int,
  Float,
  Bool,
  String,
}
