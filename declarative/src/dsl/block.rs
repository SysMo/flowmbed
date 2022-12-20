use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use super::peripheral::PeripheralRef;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockInput {
  block: String,
  id: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockOutput {
  block: String,
  id: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum Value {
  Int(i64),
  Float(f64),
  Bool(bool),
  String(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockInstance {
  id: String,
  kind: String,
  #[serde(default)]
  parameters: HashMap<String, Value>,
  #[serde(default)]
  peripherals: Vec<PeripheralRef>
}