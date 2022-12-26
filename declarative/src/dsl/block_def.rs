use std::{collections::HashMap, str::FromStr};
use serde::{Serialize, Deserialize};
use super::{FieldType, FieldValue};
use crate::util::{serde_helpers as sh};

#[derive(Debug, PartialEq, Serialize, Deserialize)] 
#[serde(deny_unknown_fields)]
pub struct BlockDefinition {
  name: String,
  #[serde(default)]
  parameters: Vec<ParameterDefinition>,
  #[serde(default)]
  inputs: Vec<InputDefinition>,
  #[serde(default)]
  outputs: Vec<OutputDefinition>,
  #[serde(default)]
  discrete_states: Vec<DiscreteStateDefinition>,
}

type FieldDefinitionVec = Vec<FieldValue>;
type FieldDefinitionMap = HashMap<String, FieldValue>;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum FieldAttrs {  
  Vec(FieldDefinitionVec),
  Map(FieldDefinitionMap)
}

impl FieldAttrs {
  pub fn into_attr_map(self) -> FieldDefinitionMap {
    match self {
      FieldAttrs::Vec(x) => {
        let mut res = HashMap::new();
        if x.len() > 0 { res.insert("type".to_owned(), x[0].clone()); }
        if x.len() > 1 { res.insert("default".to_owned(), x[1].clone()); }
        res
      },
      FieldAttrs::Map(x) => x
    }
  }
}

type FieldDef = HashMap<String, FieldAttrs>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "FieldDef")]
pub struct ParameterDefinition {
  name: String,
  tpe: FieldType,
  default: FieldValue
}

impl TryFrom<FieldDef> for ParameterDefinition {
  type Error = anyhow::Error;

  fn try_from(value: FieldDef) -> Result<Self, Self::Error> {
    let (name, attr) = sh::map2tuple(value)?;
    let attr_map = attr.into_attr_map();
    let tpe_val = attr_map.get("type").ok_or(anyhow::anyhow!("type required for parameter definition"))?;
    let tpe = FieldType::from_str(&tpe_val.as_text())?;
    let default = attr_map.get("default")
      .ok_or(anyhow::anyhow!("default value required for parameter definiction"))?.clone();
    Ok(ParameterDefinition {
      name, tpe, default,
    })    
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "FieldDef")]
pub struct InputDefinition {
  name: String,
  tpe: FieldType,  
}

impl TryFrom<FieldDef> for InputDefinition {
  type Error = anyhow::Error;

  fn try_from(value: FieldDef) -> Result<Self, Self::Error> {
    let (name, attr) = sh::map2tuple(value)?;
    let attr_map = attr.into_attr_map();
    let tpe_val = attr_map.get("type").ok_or(anyhow::anyhow!("type required for input definition"))?;
    let tpe = FieldType::from_str(&tpe_val.as_text())?;
    Ok(InputDefinition {
      name, tpe,
    })    
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "FieldDef")]
pub struct OutputDefinition {
  name: String,
  tpe: FieldType,
  default: FieldValue
}

impl TryFrom<FieldDef> for OutputDefinition {
  type Error = anyhow::Error;

  fn try_from(value: FieldDef) -> Result<Self, Self::Error> {
    let (name, attr) = sh::map2tuple(value)?;
    let attr_map = attr.into_attr_map();
    let tpe_val = attr_map.get("type").ok_or(anyhow::anyhow!("type required for output definition"))?;
    let tpe = FieldType::from_str(&tpe_val.as_text())?;
    let default = attr_map.get("default")
      .ok_or(anyhow::anyhow!("default value required for output definition"))?.clone();
    Ok(OutputDefinition {
      name, tpe, default,
    })    
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "FieldDef")]
pub struct DiscreteStateDefinition {
  name: String,
  tpe: FieldType,
  initial: FieldValue  
}

impl TryFrom<FieldDef> for DiscreteStateDefinition {
  type Error = anyhow::Error;

  fn try_from(value: FieldDef) -> Result<Self, Self::Error> {
    let (name, attr) = sh::map2tuple(value)?;
    let attr_map = attr.into_attr_map();
    let tpe_val = attr_map.get("type").ok_or(anyhow::anyhow!("type required for discrete state definition"))?;
    let tpe = FieldType::from_str(&tpe_val.as_text())?;
    let initial = attr_map.get("initial").or(attr_map.get("default"))
      .ok_or(anyhow::anyhow!("Initial value required for discrete state definition"))?.clone();
    Ok(DiscreteStateDefinition {
      name, tpe, initial,
    })    
  }
}