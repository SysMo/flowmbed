use std::{collections::HashMap};
use serde::{Serialize, Deserialize};
use super::{FieldType, FieldValue, rust::{RustTypeRef, StructTypeRef, TraitTypeRef}};
// use crate::util::{serde_helpers as sh};
use crate::util::QualifiedPath;


#[derive(Debug, PartialEq, Serialize, Deserialize)] 
#[serde(deny_unknown_fields)]
pub struct BlockModule {
  #[serde(default)]
  pub blocks: Vec<BlockDefinition>,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)] 
#[serde(deny_unknown_fields)]
pub struct BlockDefinition {
  pub name: String,
  #[serde(default)]
  pub structural: Vec<StructuralDefinition>,
  #[serde(default)]
  pub parameters: Vec<ParameterDefinition>,
  #[serde(default)]
  pub inputs: Vec<InputDefinition>,
  #[serde(default)]
  pub outputs: Vec<OutputDefinition>,
  #[serde(default)]
  pub discrete_states: Vec<DiscreteStateDefinition>,
  #[serde(default)]
  pub peripherals: Vec<PeripheralReference>,
}

// type FieldDefinitionVec = Vec<FieldValue>;
// type FieldDefinitionMap = HashMap<String, FieldValue>;

// #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
// #[serde(untagged)]
// enum FieldAttrs {  
//   Vec(FieldDefinitionVec),
//   Map(FieldDefinitionMap)
// }

// impl FieldAttrs {
//   pub fn into_attr_map(self) -> FieldDefinitionMap {
//     match self {
//       FieldAttrs::Vec(x) => {
//         let mut res = HashMap::new();
//         if x.len() > 0 { res.insert("type".to_owned(), x[0].clone()); }
//         if x.len() > 1 { res.insert("default".to_owned(), x[1].clone()); }
//         res
//       },
//       FieldAttrs::Map(x) => x
//     }
//   }
// }

// type FieldDef = HashMap<String, FieldAttrs>;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StructuralDefinition {
  Type(StructuralType),
  Constant(StructuralConstant)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralType {
  pub name: String,
  #[serde(default)]
  pub restrictions: Vec<String>
  // pub default: Option<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralConstant {
  pub name: String,
  // pub default: Option<usize>
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
// #[serde(try_from = "FieldDef")]
#[serde(deny_unknown_fields)]
pub struct ParameterDefinition {
  pub name: String,
  #[serde(rename = "type")]
  pub tpe: FieldType,
  pub default: FieldValue
}

// impl TryFrom<FieldDef> for ParameterDefinition {
//   type Error = anyhow::Error;

//   fn try_from(value: FieldDef) -> Result<Self, Self::Error> {
//     let (name, attr) = sh::map2tuple(value)?;
//     let attr_map = attr.into_attr_map();
//     let tpe_val = attr_map.get("type").ok_or(anyhow::anyhow!("type required for parameter definition"))?;
//     let tpe = FieldType::from_str(&tpe_val.as_text())?;
//     let default = attr_map.get("default")
//       .ok_or(anyhow::anyhow!("default value required for parameter definiction"))?.clone();
//     Ok(ParameterDefinition {
//       name, tpe, default,
//     })    
//   }
// }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
// #[serde(try_from = "FieldDef")]
#[serde(deny_unknown_fields)]
pub struct InputDefinition {
  pub name: String,
  #[serde(rename = "type")]
  pub tpe: FieldType,
  pub size: Option<FieldValue>
}

// impl TryFrom<FieldDef> for InputDefinition {
//   type Error = anyhow::Error;

//   fn try_from(value: FieldDef) -> Result<Self, Self::Error> {
//     let (name, attr) = sh::map2tuple(value)?;
//     let attr_map = attr.into_attr_map();
//     let tpe_val = attr_map.get("type").ok_or(anyhow::anyhow!("type required for input definition"))?;
//     let tpe = FieldType::from_str(&tpe_val.as_text())?;
//     let size = attr_map.get("size").map(|x| x.to_owned());
//     Ok(InputDefinition {
//       name, tpe, size
//     })    
//   }
// }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
// #[serde(try_from = "FieldDef")]
#[serde(deny_unknown_fields)]
pub struct OutputDefinition {
  pub name: String,
  #[serde(rename = "type")]
  pub tpe: FieldType,
  pub default: Option<FieldValue>,
  pub size: Option<FieldValue>
}

// impl TryFrom<FieldDef> for OutputDefinition {
//   type Error = anyhow::Error;

//   fn try_from(value: FieldDef) -> Result<Self, Self::Error> {
//     let (name, attr) = sh::map2tuple(value)?;
//     let attr_map = attr.into_attr_map();
//     let tpe_val = attr_map.get("type").ok_or(anyhow::anyhow!("type required for output definition"))?;
//     let tpe = FieldType::from_str(&tpe_val.as_text())?;
//     let default = attr_map.get("default")
//       .ok_or(anyhow::anyhow!("default value required for output definition"))?.clone();
//     let size = attr_map.get("size").map(|x| x.to_owned());
//     Ok(OutputDefinition {
//       name, tpe, default, size
//     })    
//   }
// }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
// #[serde(try_from = "FieldDef")]
#[serde(deny_unknown_fields)]
pub struct DiscreteStateDefinition {
  pub name: String,
  #[serde(rename = "type")]
  pub tpe: FieldType,
  pub initial: FieldValue  
}

// impl TryFrom<FieldDef> for DiscreteStateDefinition {
//   type Error = anyhow::Error;

//   fn try_from(value: FieldDef) -> Result<Self, Self::Error> {
//     let (name, attr) = sh::map2tuple(value)?;
//     let attr_map = attr.into_attr_map();
//     let tpe_val = attr_map.get("type").ok_or(anyhow::anyhow!("type required for discrete state definition"))?;
//     let tpe = FieldType::from_str(&tpe_val.as_text())?;
//     let initial = attr_map.get("initial").or(attr_map.get("default"))
//       .ok_or(anyhow::anyhow!("Initial value required for discrete state definition"))?.clone();
//     Ok(DiscreteStateDefinition {
//       name, tpe, initial,
//     })    
//   }
// }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PeripheralReference {
  Type(PeripheralReferenceType),
  Trait(PeripheralReferenceTrait),  
}

impl PeripheralReference {
  pub fn name<'a>(&self) -> &str {
    match self {
        PeripheralReference::Type(x) => &x.name,
        PeripheralReference::Trait(x) => &x.name,
    }
  }

  pub fn direction<'a>(&self) -> &str {
    match self {
        PeripheralReference::Type(x) => &x.direction,
        PeripheralReference::Trait(x) => &x.direction,
    }
  }

  pub fn mut_ref(&self, lifetime: Option<&str>) -> RustTypeRef {
    //let mut gats = HashMap::new();
    // gats.insert("Error".to_owned(), "HalError".to_owned().into());
    match self {
        PeripheralReference::Type(x) => 
          RustTypeRef::Struct(StructTypeRef {
            qpath: x.qpath.clone(),
            mutable: true,
            lifetime: lifetime.map(|x| x.to_owned())
        }),
        PeripheralReference::Trait(x) =>           
        RustTypeRef::Trait(TraitTypeRef {
            qpath: x.qpath.clone(),
            mutable: true,
            lifetime: lifetime.map(|x| x.to_owned()),
            gats: HashMap::new()
        }),
    }
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeripheralReferenceType {
  pub name: String,
  pub direction: String,
  #[serde(rename = "type")]
  pub qpath: QualifiedPath,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeripheralReferenceTrait {
  pub name: String,
  pub direction: String,
  #[serde(rename = "trait")]
  pub qpath: QualifiedPath,
}