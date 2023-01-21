#[allow(unused_macros)]
macro_rules! block_builder_field {

  ($block: ident, $field: ident, Parameter, $tpe: ty) => {
    pub fn $field(mut self, v: $tpe) -> Self {
      self.component.$field.reset(v);
      self
    }
  };

  ($block: ident, $field: ident, $field_type: ident, $tpe: ty) => {
    compile_error!(concat!("Unknown field type, expected Parameter"));
  };
}

#[allow(unused_macros)]
macro_rules! block_builder {
  ($block: ident, $([$field: ident, $field_type: ident, $tpe: ty])+) => {
    pub struct Builder<'a> {
      component: $block<'a>
    }

    impl<'a> Builder<'a> {
      $(block_builder_field!($block, $field, $field_type, $tpe);)+
    }

    impl<'a> From<Builder<'a>> for $block<'a> {
      fn from(x: Builder<'a>) -> Self {
        x.component
      }
    }    
  };

}

#[allow(unused_imports)]
pub(crate) use {block_builder, block_builder_field};