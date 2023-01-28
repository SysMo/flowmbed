use serde::{Serialize, Deserialize};
use crate::dsl::device::{PeripheralConfig};
use crate::gen::device::PeripheralConfigGenerator;
use crate::util::GenerationContext;
use genco::prelude::{rust, quote};
use super::super::IMPORTS;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SerialValueSink {

}

#[typetag::serde]
impl PeripheralConfig for SerialValueSink {}

impl PeripheralConfigGenerator for SerialValueSink {
  fn gen_type(&self, _: &GenerationContext) -> anyhow::Result<rust::Tokens> {
    let esp32hal = &IMPORTS.esp32hal;
    Ok(quote!($(esp32hal)::SerialValueSink))
  }

  fn gen_initialize(&self, _: &GenerationContext) -> anyhow::Result<rust::Tokens> {
    let esp32hal = &IMPORTS.esp32hal;
    Ok(quote!($(esp32hal)::SerialValueSink {}))
  }
}