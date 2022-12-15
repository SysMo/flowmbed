use super::system_storage::{StorageSize, StorageAccess, SystemStorageFacade, SystemStorageBuilder, VariableCreator, DefaultSystemStrorage};
use super::variables::{Parameter, DiscreteState, ContinuousState};
use std::cell::RefCell;

#[derive(Debug)]
pub struct HeapSystemStorage {
  size: StorageSize,
  r_param: Vec<RefCell<f64>>,
  b_param: Vec<RefCell<bool>>,
  i_param: Vec<RefCell<i64>>,
  // r_state: Vec<RefCell<f64>>,
  // r_state_der: Vec<RefCell<f64>>,
  b_state: Vec<RefCell<bool>>,
  i_state: Vec<RefCell<i64>>,
}

impl HeapSystemStorage {
  pub fn new(size: StorageSize) -> HeapSystemStorage {
    HeapSystemStorage {
      size: size.clone(),
      r_param: vec![RefCell::new(0.0); size.r_param],
      b_param: vec![RefCell::new(false); size.b_param],
      i_param: vec![RefCell::new(0); size.i_param],
      // r_state: vec![RefCell::new(0.0); size.r_state],
      // r_state_der: vec![RefCell::new(0.0); size.r_state],
      b_state: vec![RefCell::new(false); size.b_state],
      i_state: vec![RefCell::new(0); size.i_state],
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
          unsafe {&*self.$field[ind].as_ptr()}
        }
      
        // TODO See if RefCell can be avoided and self can be made mutable here
        fn set(&self, ind: usize, value: $tpe) -> anyhow::Result<()> {    
          *self.$field[ind].borrow_mut() = value;
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
heap_storage_impl_all!(Parameter, i64, i_param);
heap_storage_impl_all!(Parameter, bool, b_param);
// heap_storage_impl_all!(ContinuousState<'a, f64>, f64, r_state);
// heap_storage_impl_access!(ContinuousStateDerivative<'a, f64>, f64, r_state_der);
heap_storage_impl_all!(DiscreteState, bool, b_state);
heap_storage_impl_all!(DiscreteState, i64, i_state);

// impl SystemStorageFacade for HeapSystemStorage {
//   fn create_parameter<'a, T: Copy>(&'a self) -> Parameter<'a, T>
//   where Self: StorageAccess<'a, Parameter<'a, T>, T> {
//     Parameter::<'a, T> { id: 0, access: self }
//   }
// }

impl DefaultSystemStrorage for HeapSystemStorage {

}
