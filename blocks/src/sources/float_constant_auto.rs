use flowmbed_dynsys::core as ds_core;

#[allow(unused_imports)]
use flowmbed_dynsys::core::{Float, Int, Bool, String};

/// Declare the block struct
#[allow(dead_code)]
pub struct FloatConstant<'a> {
  // Parameters
  pub value: ds_core::Parameter<'a, ds_core::Float>,
  // Inputs
  // Outputs
  pub output: ds_core::Output<ds_core::Float>,
  // Discrete states
  // Peripherals
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> FloatConstant<'a> {
  pub fn builder() -> Builder<'a> {
    Builder {
      __phantom: std::marker::PhantomData,
      val_value: 0e0,
    }
  }
}

#[allow(non_snake_case)]
pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  val_value: ds_core::Float,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {
  pub fn value(mut self, v: ds_core::Float) -> Self {
    self.val_value = v;
    self
  }
}

#[allow(unused_variables)]
impl<'a> ds_core::BlockBuilder<'a, FloatConstant<'a>> for Builder<'a> {
  fn build<ST: ds_core::DefaultSystemStrorage>(self, storage_builder: &mut ds_core::SystemStorageBuilder<'a, ST>) -> FloatConstant<'a> {
    FloatConstant {
      value: storage_builder.create_param(self.val_value),

      output: ds_core::Output::new(0e0),

    }
  }
}

impl<'a> ds_core::RequiresStorage for FloatConstant<'a> {
  const SIZE: ds_core::StorageSize = ds_core::StorageSize {
    r_param: 1, b_param: 0, i_param: 0,

    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
