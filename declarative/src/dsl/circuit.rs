use serde::{Serialize, Deserialize};
use super::block_instance::{BlockInstance, BlockInput, BlockOutput};
use std::collections::HashMap;
use crate::util::{serde_helpers as sh};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CircuitConfig {
  pub id: String,
  pub device: String,
  pub blocks: Vec<BlockInstance>,
  // pub connections: Vec<BlockConnection>,
}

type ConnectionMap = HashMap<String, String>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ConnectionMap")]
pub struct BlockConnection {
  pub to: BlockInput,
  pub from: BlockOutput,
}

impl TryFrom<ConnectionMap> for BlockConnection {
  type Error = anyhow::Error;

  fn try_from(cm: ConnectionMap) -> Result<Self, Self::Error> {
    let conn_tupple = sh::map2tuple(cm)?;
    let from_tupple: [&str; 2] = 
      sh::to_array(&mut conn_tupple.1.split("."))?;
    let to_tupple: [&str; 2] = 
      sh::to_array(&mut conn_tupple.0.split("."))?;

    let to = BlockInput { 
      block: to_tupple[0].to_owned(), 
      id: to_tupple[1].to_owned() 
    };
    let from = BlockOutput { 
      block: from_tupple[0].to_owned(), 
      id: from_tupple[1].to_owned() 
    };
    Ok(BlockConnection {to, from})
  }
}