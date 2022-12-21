use serde::{Serialize, Deserialize};
use super::block::{BlockInstance, BlockInput, BlockOutput};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockConnection {
  pub to: BlockInput,
  pub from: BlockOutput,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CircuitConfig {
  pub id: String,
  pub blocks: Vec<BlockInstance>
}