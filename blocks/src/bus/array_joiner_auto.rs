use flowmbed_dynsys::core as ds_core;

/// Declare the block struct
#[allow(dead_code)]
pub struct ArrayJoiner<'a, T: Copy + Default, const N: usize> {
  // Inputs
  pub inputs: [ds_core::Input<'a, T>; N],
  // Outputs
  pub output: ds_core::Output<[T; N]>,
  // Discrete states
  // Peripherals
}

/// Implement the block struct
#[allow(dead_code)]
impl<'a, T: Copy + Default, const N: usize> ArrayJoiner<'a, T, N> {
  pub fn builder() -> Builder<'a, T, N> {
    Builder {
      __phantom: std::marker::PhantomData,
      __phantom_T: std::marker::PhantomData,
    }
  }
}

#[allow(non_snake_case)]
pub struct Builder<'a, T: Copy + Default, const N: usize> {
  __phantom: std::marker::PhantomData<&'a ()>,
  __phantom_T: std::marker::PhantomData<T>,
}

#[allow(dead_code)]
impl<'a, T: Copy + Default, const N: usize> Builder<'a, T, N> {}

#[allow(unused_variables)]
impl<'a, T: Copy + Default, const N: usize> ds_core::BlockBuilder<'a, ArrayJoiner<'a, T, N>> for Builder<'a, T, N> {
  fn build<ST: ds_core::DefaultSystemStrorage>(self, storage_builder: &mut ds_core::SystemStorageBuilder<'a, ST>) -> ArrayJoiner<'a, T, N> {
    ArrayJoiner {

      inputs: core::array::from_fn(|_| ds_core::Input::new()),

      output: ds_core::Output::new(ds_core::create_default::<T>()),

    }
  }
}

impl<'a, T: Copy + Default, const N: usize> ds_core::RequiresStorage for ArrayJoiner<'a, T, N> {
  const SIZE: ds_core::StorageSize = ds_core::StorageSize {
    r_param: 0, b_param: 0, i_param: 0,

    r_dstate: 0, b_dstate: 0, i_dstate: 0,
  };
}
