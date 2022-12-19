use super::system_storage::{StorageSize, StorageAccess, SystemStorageFacade, SystemStorageBuilder, VariableCreator, DefaultSystemStrorage};
use super::variables::{Parameter, DiscreteState, Output};

#[derive(Debug)]
#[allow(dead_code)]
pub struct HeapSystemStorage {
  size: StorageSize,
  r_param: Vec<f64>,
  b_param: Vec<bool>,
  i_param: Vec<i64>,
  r_dstate: Vec<f64>,
  b_dstate: Vec<bool>,
  i_dstate: Vec<i64>,
  r_out: Vec<f64>,
  b_out: Vec<bool>,
  i_out: Vec<i64>,
}

impl HeapSystemStorage {
  pub fn new(size: StorageSize) -> HeapSystemStorage {
    HeapSystemStorage {
      size: size.clone(),
      r_param: vec![0.0; size.r_param],
      b_param: vec![false; size.b_param],
      i_param: vec![0; size.i_param],

      r_dstate: vec![0.0; size.r_dstate],
      b_dstate: vec![false; size.b_dstate],
      i_dstate: vec![0; size.i_dstate],

      r_out: vec![0.0; size.r_out],
      b_out: vec![false; size.b_out],
      i_out: vec![0; size.i_out],
    }
  }
}

impl SystemStorageFacade for HeapSystemStorage {
  fn size(&self) -> StorageSize {
    self.size.clone()
  }
}

macro_rules! heap_storage_impl_access {
    ($kind: ident, $tpe: ty, $field: ident) => {
      impl<'a> StorageAccess<'a, $kind<'a, $tpe>, $tpe> for HeapSystemStorage {
        fn get(&'a self, ind: usize) -> &'a $tpe {
          &self.$field[ind]
          // unsafe {&*self.$field[ind].as_ptr()}
        }
      
        // TODO See if RefCell can be avoided and self can be made mutable here
        fn set(&self, ind: usize, value: $tpe) -> anyhow::Result<()> {
          let mut_self: *const Self = self;
          let mut_self2: *mut Self = mut_self as *mut Self;
          let mut_self3: &mut Self = unsafe {&mut *mut_self2};
          mut_self3.$field[ind] = value;
          Ok(())
        }
      }
      
    };    
}

macro_rules! heap_storage_create_var {
    ($kind: ident, $tpe: ty, $field: ident) => {
      impl<'a> VariableCreator<'a, $kind<'a, $tpe>, $tpe> for SystemStorageBuilder<'a, HeapSystemStorage> {
        fn create(&mut self) -> $kind<'a, $tpe> {
          let current_index = self.counters.$field;
          self.counters.$field += 1;
          $kind::<$tpe> { id: current_index, access: self.storage }
        }
      }
    };
}

macro_rules! heap_storage_impl_all {
    ($kind: ident, $tpe: ty, $field: ident) => {
      heap_storage_impl_access!($kind, $tpe, $field);
      heap_storage_create_var!($kind, $tpe, $field);
    };
}

heap_storage_impl_all!(Parameter, f64, r_param);
heap_storage_impl_all!(Parameter, bool, b_param);
heap_storage_impl_all!(Parameter, i64, i_param);

heap_storage_impl_all!(DiscreteState, f64, r_dstate);
heap_storage_impl_all!(DiscreteState, bool, b_dstate);
heap_storage_impl_all!(DiscreteState, i64, i_dstate);

heap_storage_impl_all!(Output, f64, r_param);
heap_storage_impl_all!(Output, bool, b_param);
heap_storage_impl_all!(Output, i64, i_param);

// heap_storage_impl_all!(ContinuousState<'a, f64>, f64, r_state);
// heap_storage_impl_access!(ContinuousStateDerivative<'a, f64>, f64, r_state_der);

// impl SystemStorageFacade for HeapSystemStorage {
//   fn create_parameter<'a, T: Copy>(&'a self) -> Parameter<'a, T>
//   where Self: StorageAccess<'a, Parameter<'a, T>, T> {
//     Parameter::<'a, T> { id: 0, access: self }
//   }
// }

impl DefaultSystemStrorage for HeapSystemStorage {

}
