use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// use super::device::PeripheralRef;
use super::FieldValue;

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
#[serde(deny_unknown_fields)]
pub struct BlockInstance {
  pub id: String,
  pub kind: String,
  #[serde(default)]
  pub parameters: HashMap<String, FieldValue>,
  #[serde(default)]
  pub peripherals: HashMap<String, String>
}