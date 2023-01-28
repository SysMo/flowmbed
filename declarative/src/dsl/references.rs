use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::util::QualifiedPath;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageImport {
  pub package: String,
  pub paths: Vec<NamespaceImport>,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum StrNameOrMap {
  Name(QualifiedPath),
  Map(HashMap<QualifiedPath, String>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "StrNameOrMap")]
pub struct NamespaceImport {
  pub qpath: QualifiedPath,
  pub alias: Option<String>
}

impl TryFrom<StrNameOrMap> for NamespaceImport {
  type Error = anyhow::Error;

  fn try_from(cm: StrNameOrMap) -> Result<Self, Self::Error> {
    match cm {
      StrNameOrMap::Name(qpath) => Ok(NamespaceImport {
        qpath, alias: None
      }),
      StrNameOrMap::Map(map) => {
        if map.len() == 1 {
          let qpath = map.keys().last().unwrap().to_owned();
          let alias = Some(map.get(&qpath).unwrap().to_owned());
          Ok(NamespaceImport { qpath, alias })
        } else {
          anyhow::bail!("Expected string or mapping (key: value)")
        }
      }
    }
  }
}



