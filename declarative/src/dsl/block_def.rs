use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::{FieldType, FieldValue};
use crate::util::{serde_helpers as sh};

#[derive(Debug, PartialEq, Serialize, Deserialize)] 
#[serde(deny_unknown_fields)]
pub struct BlockDefinition {
  name: String,
  parameters: Vec<ParameterDefinition>
}

type ParameterDefinitionMap = HashMap<String, Vec<FieldValue>>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ParameterDefinitionMap")]
pub struct ParameterDefinition {
  name: String,
  tpe: FieldType,
  default: FieldValue
}

impl TryFrom<ParameterDefinitionMap> for ParameterDefinition {
  type Error = anyhow::Error;

  fn try_from(value: ParameterDefinitionMap) -> Result<Self, Self::Error> {
    // let param_tuple = sh::map2tuple(&value)?;
    // let param_attr: [FieldValue; 2] = sh::to_array(&param_tuple.1.iter())?;
    Ok(ParameterDefinition {
      name: "asa".to_owned(), 
      tpe: FieldType::Int, 
      default: FieldValue::Int(1),
    })    
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)] 
pub struct InputDefinition {
  name: String,
  tpe: FieldValue,  
}

#[derive(Debug, PartialEq, Serialize, Deserialize)] 
pub struct OutputDefinition {
  name: String,
  tpe: FieldType,
  default: FieldValue
}

#[derive(Debug, PartialEq, Serialize, Deserialize)] 
pub struct DiscreteState {
  name: String,
  tpe: FieldType,
  initial: FieldValue  
}