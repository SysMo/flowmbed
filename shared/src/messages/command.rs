use serde::{Serialize, Deserialize};
use super::dynsys::ParamSet;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
  ParamSet(ParamSet)
}

impl Into<Command> for ParamSet {
  fn into(self) -> Command {
    Command::ParamSet(self)
  }
} 