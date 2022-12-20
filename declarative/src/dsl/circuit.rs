use serde::{Serialize, Deserialize};
use super::block::{BlockInstance, BlockInput, BlockOutput};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockConnection {
  to: BlockInput,
  from: BlockOutput,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Circuit {
  id: String,
  blocks: Vec<BlockInstance>
}