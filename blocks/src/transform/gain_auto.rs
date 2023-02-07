use flowmbed_dynsys::core as ds_core;

#[allow(unused_imports)]
use flowmbed_dynsys::core::{Float, Int, Bool, String};

/// Declare the block struct
#[allow(dead_code)]
pub struct Gain<'a> {
  // Parameters
  pub gain: ds_core::Parameter<'a, ds_core::Float>,
  // Inputs
  pub input: ds_core::Input<'a, ds_core::Float>,
  // Outputs
  pub output: ds_core::Output<ds_core::Float>,
  // Discrete states
  // Peripherals
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a> Gain<'a> {
  pub fn builder() -> Builder<'a> {
    Builder {
      __phantom: std::marker::PhantomData,
      val_gain: 1e0,
    }
  }
}

#[allow(non_snake_case)]
pub struct Builder<'a> {
  __phantom: std::marker::PhantomData<&'a ()>,
  val_gain: ds_core::Float,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {
  pub fn gain(mut self, v: ds_core::Float) -> Self {
    self.val_gain = v;
    self
  }
}

#[allow(unused_variables)]
impl<'a> ds_core::BlockBuilder<'a, Gain<'a>> for Builder<'a> {
  fn build<ST: ds_core::DefaultSystemStrorage>(self, storage_builder: &mut ds_core::SystemStorageBuilder<'a, ST>) -> Gain<'a> {
    Gain {
      gain: storage_builder.create_param(self.val_gain),

      input: ds_core::Input::new(),

      output: ds_core::Output::new(ds_core::create_default::<Float>()),

    }
  }
}

impl<'a> ds_core::RequiresStorage for Gain<'a> {
  const SIZE: ds_core::StorageSize = ds_core::StorageSize {
    r_param: 1, b_param: 0, i_param: 0,

    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
