use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ParamSet {
  Real(u32, f64),
  Int(u32, u64),
  Bool(u32, bool)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ParamGet {
  Real(u32),
  Int(u32),
  Bool(u32)
}

