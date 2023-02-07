use genco::prelude::rust;

use crate::util::context::GenerationContext;

pub trait CodeGenerator {
  fn generate(&self, context: &dyn GenerationContext) -> anyhow::Result<rust::Tokens>;
}
