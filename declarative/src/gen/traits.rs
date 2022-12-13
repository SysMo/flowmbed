use genco::prelude::rust;
use crate::dsl::board::BoardConfig;

pub trait Generate {
  fn gen(self) -> rust::Tokens;
}