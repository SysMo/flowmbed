use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use super::device::PeripheralRef;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockInput {
  pub block: String,
  pub id: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockOutput {
  pub block: String,
  pub id: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
  Int(i64),
  Float(f64),
  Bool(bool),
  String(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BlockInstance {
  pub id: String,
  pub kind: String,
  #[serde(default)]
  pub parameters: HashMap<String, Value>,
  #[serde(default)]
  pub peripherals: Vec<PeripheralRef>
}