use genco::prelude::rust;

use crate::util::GenerationContext;

pub trait CodeGenerator {
  fn generate(&self, context: &GenerationContext) -> anyhow::Result<rust::Tokens>;
}
