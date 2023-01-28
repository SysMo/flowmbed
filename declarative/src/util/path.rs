use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(from = "String")]
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

  pub fn append(&self, segment: &str) -> QualifiedPath {
    let new_capacity = self.segments.len() + 1;
    let mut new_segments = Vec::<String>::with_capacity(new_capacity);
    for i in 0..(new_capacity - 1)  {
      new_segments.push(self.segments[i].clone());
    }
    new_segments.push(segment.to_owned());

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

impl From<String> for QualifiedPath {
  fn from(s: String) -> Self {
    let segments: Vec<String> = s.split("::").map(|x| x.to_owned()).collect();
    QualifiedPath { segments }
  }
}