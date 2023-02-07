use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::util::QualifiedPath;

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
  pub structural: Vec<FieldValue>,
  #[serde(default)]
  pub parameters: HashMap<String, FieldValue>,
  #[serde(default)]
  pub inputs: HashMap<InputRef, OutputRef>,
  #[serde(default)]
  pub peripherals: HashMap<String, String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize, std::cmp::Eq, Hash)]
#[serde(try_from = "String")]
pub struct InputRef {
  pub input_id: String
}

impl TryFrom<String> for InputRef {
  type Error = anyhow::Error;
  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(InputRef { input_id: value })
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub struct OutputRef {
  pub block_id: String,
  pub output_id: String,
}

impl TryFrom<String> for OutputRef {
  type Error = anyhow::Error;
  fn try_from(value: String) -> Result<Self, Self::Error> {
    let path: QualifiedPath = QualifiedPath::parse(&value, ".");
    if path.len() == 2 {
      Ok(OutputRef { 
        block_id: path.segments[0].to_owned(), 
        output_id: path. segments[1].to_owned()
      })      
    } else {
      anyhow::bail!("Output ref {} should be of the form component_id.output_id", value)
    }
  }
}