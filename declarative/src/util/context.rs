use crate::dsl::device::PeripheralConfig;

pub struct GenerationContext<'a> {
  pub id: String,
  pub object: ContextObjectEnum<'a>,
  pub parent: Option<&'a GenerationContext<'a>>
}

impl<'a> GenerationContext<'a> {
  pub fn root() -> Self {
    GenerationContext { 
      id: "".to_owned(), object: ContextObjectEnum::Root, parent: None 
    }
  }

  pub fn push(&'a self, id: &str, object: ContextObjectEnum<'a>) -> Self {
    GenerationContext { 
      id: id.to_owned(), object , parent: Some(self)
    }
  }
}


pub enum ContextObjectEnum<'a> {
  Root,
  Device,
  Peripheral(&'a dyn PeripheralConfig)
}