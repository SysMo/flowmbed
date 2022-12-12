use serde::{Serialize, Deserialize};
use uuid::Uuid;
use super::dynsys::ParamGet;

#[derive(Serialize, Deserialize, Debug)]
pub enum Query {
  ParamGet(ParamGet)
}

pub fn gen_id() -> u128 {
  Uuid::new_v4().as_u128()
}

impl Into<Query> for ParamGet {
  fn into(self) -> Query {
     Query::ParamGet(self)
  }
} 