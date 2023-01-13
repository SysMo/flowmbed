use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(try_from = "String")]
pub struct QualifiedPath {
  pub segments: Vec<String>
}

impl QualifiedPath {
  pub fn len(&self) -> usize {
    self.segments.len()
  }

  pub fn prepend(&self, segment: &str) -> QualifiedPath {
    let new_capacity = self.segments.len() + 1;
    let mut new_segments = Vec::<String>::with_capacity(new_capacity);
    new_segments.push(segment.to_owned());
    for i in 1..new_capacity {
      new_segments.push(self.segments[i - 1].clone());
    }

    QualifiedPath {
      segments: new_segments
    }
  }

  pub fn root<'a>(&'a self) -> &'a str {
    &self.segments[0]
  }

  pub fn rest(&self) -> Option<QualifiedPath> {
    if self.segments.len() > 1 {
      let new_segments: Vec<String> = self.segments[1..self.segments.len()]
        .into_iter().map(|x| x.to_owned()).collect();
      Some(QualifiedPath {segments: new_segments})
    } else {
      None
    }
  }


  pub fn parent(&self) -> Option<QualifiedPath> {
    if self.segments.len() > 1 {
      let new_segments: Vec<String> = self.segments[0..(self.segments.len() - 1)]
        .into_iter().map(|x| x.to_owned()).collect();
      Some(QualifiedPath {segments: new_segments})
    } else {
      None
    }    
  }


  pub fn name<'a>(&'a self) -> &'a str {
    &self.segments.last().unwrap()
  }
}

impl ToString for QualifiedPath {
  fn to_string(&self) -> String {
      self.segments.join("::")
  }
}

impl std::hash::Hash for QualifiedPath {
  fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
    self.to_string().hash(state)
  }
}

impl PartialEq for QualifiedPath {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for QualifiedPath {}

impl TryFrom<String> for QualifiedPath {
  type Error = anyhow::Error;

  fn try_from(s: String) -> Result<Self, Self::Error> {
    let segments: Vec<String> = s.split("::").map(|x| x.to_owned()).collect();
    Ok(QualifiedPath { segments })
  }
}
