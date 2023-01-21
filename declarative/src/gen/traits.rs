use genco::prelude::rust;

pub trait CodeGenerator {
  fn generate(&self) -> anyhow::Result<rust::Tokens>;
}
