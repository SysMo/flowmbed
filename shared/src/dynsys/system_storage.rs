use const_default::ConstDefault;
use const_default_derive::ConstDefault;
use std::cell::{RefCell, Ref};
use super::variables::{Parameter, ContinuousState, ContinuousStateDerivative, DiscreteState};

#[allow(dead_code)]
#[derive(Default, Debug, ConstDefault, PartialEq, Eq, Clone)]
pub struct StorageSize {
    pub r_param: usize,
    pub b_param: usize,
    pub r_state: usize,
    pub b_state: usize,
    pub i_state: usize,
    pub r_out: usize,
    pub b_out: usize,
}

pub trait NextIndex<T> {
  fn next_index(&mut self) -> usize;
}

impl<'a> NextIndex<Parameter<'a, f64>> for StorageSize {
  fn next_index(&mut self) -> usize {
      let current = self.r_param;
      self.r_param += 1;
      current
  }
}

impl<'a> NextIndex<DiscreteState<'a, bool>> for StorageSize {
  fn next_index(&mut self) -> usize {
      let current = self.b_state;
      self.r_param += 1;
      current
  }
}

impl<'a> NextIndex<DiscreteState<'a, i64>> for StorageSize {
  fn next_index(&mut self) -> usize {
      let current = self.i_state;
      self.i_state += 1;
      current
  }
}


#[allow(dead_code)]
impl StorageSize {
    pub const fn new() -> StorageSize {
        StorageSize::DEFAULT
    }
}

pub trait StorageAccess<'a, VarKind, T> {
  fn get(&'a self, ind: usize) -> &'a T;
  fn set(&self, ind: usize, value: T) -> anyhow::Result<()>;
}
pub trait SystemStorageFacade {
  fn size(&self) -> StorageSize;

  fn get_value<'a, K, T>(&'a self, ind: usize) -> &T 
  where Self: StorageAccess<'a, K, T> {
    <Self as StorageAccess<'a, K, T>>::get(&self, ind)
  }

  fn set_value<'a, K, T>(&'a self, ind: usize, v: T) -> anyhow::Result<()>
  where Self: StorageAccess<'a, K, T> {
    <Self as StorageAccess<'a, K, T>>::set(&self, ind, v)
  }

  fn set_parameter<'a, T : 'a>(&'a self, ind: usize, v: T) -> anyhow::Result<()>
  where Self: StorageAccess<'a, Parameter<'a, T>, T> {
    self.set_value::<Parameter<T>, T>(ind, v)    
  }

  fn set_cont_state<'a, T : 'a>(&'a self, ind: usize, v: T) -> anyhow::Result<()>
  where Self: StorageAccess<'a, ContinuousState<'a, T>, T> {
    self.set_value::<ContinuousState<T>, T>(ind, v)
  }

  fn set_discrete_state<'a, T : 'a>(&'a self, ind: usize, v: T) -> anyhow::Result<()>
  where Self: StorageAccess<'a, DiscreteState<'a, T>, T> {
    self.set_value::<DiscreteState<T>, T>(ind, v)
  }
}

#[derive(Debug)]
pub struct HeapSystemStorage {
  size: StorageSize,
  r_param: Vec<RefCell<f64>>,
  r_state: Vec<RefCell<f64>>,
  r_state_der: Vec<RefCell<f64>>,
  b_state: Vec<RefCell<bool>>,
  i_state: Vec<RefCell<i64>>,
}

impl HeapSystemStorage {
  pub fn new(size: StorageSize) -> HeapSystemStorage {
    HeapSystemStorage {
      size: size.clone(),
      r_param: vec![RefCell::new(0.0); size.r_param],
      r_state: vec![RefCell::new(0.0); size.r_state],
      r_state_der: vec![RefCell::new(0.0); size.r_state],
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


pub struct SystemStorageBuilder<'a, ST> {
  size: StorageSize,
  storage: &'a ST,
  counters: StorageSize
}

impl<'a, ST: SystemStorageFacade> SystemStorageBuilder<'a, ST> {
  pub fn new(storage:&'a ST) -> SystemStorageBuilder<'a, ST> {
    SystemStorageBuilder {
      size: storage.size(),
      storage: storage,
      counters: StorageSize::DEFAULT,
    }
  }

  //  + StorageSize: NextIndex<Parameter<'a, T>>
  pub fn create_param<T>(&mut self, default: T) -> Parameter<'a, T> 
  where 
    ST: StorageAccess<'a, Parameter<'a, T>, T>,
    StorageSize: NextIndex<Parameter<'a, T>>
  {
    let next_index = (&mut self.counters as &mut dyn NextIndex<Parameter<'a, T>>).next_index();
    let param = Parameter { id: next_index, access: self.storage };
    self.storage.set_parameter(param.id, default).unwrap();
    param
  }

  pub fn create_discrete_state<T>(&mut self, initial: T) -> DiscreteState<'a, T>  
  where 
  ST: StorageAccess<'a, DiscreteState<'a, T>, T> ,
  StorageSize: NextIndex<DiscreteState<'a, T>>
  {
    let next_index = (&mut self.counters as &mut dyn NextIndex<DiscreteState<'a, T>>).next_index();
    let state = DiscreteState { id: next_index, access: self.storage };
    self.storage.set_discrete_state(state.id, initial).unwrap();
    state
  }
}

pub trait VariableCreator<'a, K, T> {  
  fn create(&mut self) -> K;
}

macro_rules! heap_storage_impl_access {
    ($kind: path, $tpe: ty, $field: ident) => {
      impl<'a> StorageAccess<'a, $kind, $tpe> for HeapSystemStorage {
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
    ($kind: path, $tpe: ty, $field: ident) => {
      impl<'a> VariableCreator<'a, $kind, $tpe> for SystemStorageBuilder<'a, HeapSystemStorage> {
        fn create(&mut self) -> $kind {
          let current_index = self.counters.$field;
          self.counters.$field += 1;
          $kind { id: current_index, access: self.storage }
        }
      }
    };
}

macro_rules! heap_storage_impl_all {
    ($kind: path, $tpe: ty, $field: ident) => {
      heap_storage_impl_access!($kind, $tpe, $field);
      heap_storage_create_var!($kind, $tpe, $field);
    };
}

heap_storage_impl_all!(Parameter<'a, f64>, f64, r_param);
heap_storage_impl_all!(ContinuousState<'a, f64>, f64, r_state);
heap_storage_impl_access!(ContinuousStateDerivative<'a, f64>, f64, r_state_der);
heap_storage_impl_all!(DiscreteState<'a, bool>, bool, b_state);
heap_storage_impl_all!(DiscreteState<'a, i64>, i64, i_state);

// impl SystemStorageFacade for HeapSystemStorage {
//   fn create_parameter<'a, T: Copy>(&'a self) -> Parameter<'a, T>
//   where Self: StorageAccess<'a, Parameter<'a, T>, T> {
//     Parameter::<'a, T> { id: 0, access: self }
//   }
// }


