use const_default::ConstDefault;
use const_default_derive::ConstDefault;
use std::cell::{RefCell, Ref};
use super::variables::{Parameter, ContinuousState, ContinuousStateDerivative, DiscreteState};

#[allow(dead_code)]
#[derive(Default, Debug, ConstDefault, PartialEq, Eq, Clone)]
pub struct StorageSize {
    pub r_param: usize,
    pub b_param: usize,
    pub i_param: usize,

    pub r_state: usize,
    pub b_state: usize,
    pub i_state: usize,
    
    // pub r_out: usize,
    // pub b_out: usize,
}

pub trait NextIndex<T> {
  fn next_index(&mut self) -> usize;
}

macro_rules! next_index_impl {
  ($kind: ident, $tpe: ty, $field: ident) => {
    impl<'a> NextIndex<$kind<'a, $tpe>> for StorageSize {
      fn next_index(&mut self) -> usize {
        let current = self.$field;
        self.$field += 1;
        current
      }
    }
  };
}

next_index_impl!(Parameter, f64, r_param);
next_index_impl!(Parameter, bool, b_param);
next_index_impl!(Parameter, i64, i_param);
next_index_impl!(DiscreteState, bool, b_state);
next_index_impl!(DiscreteState, i64, i_state);

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


pub trait DefaultSystemStrorage : SystemStorageFacade 
where for<'a> Self: 
  StorageAccess<'a, Parameter<'a, f64>, f64> + 
  StorageAccess<'a, Parameter<'a, bool>, bool> +
  StorageAccess<'a, Parameter<'a, i64>, i64> +
  StorageAccess<'a, DiscreteState<'a, bool>, bool> +
  StorageAccess<'a, DiscreteState<'a, i64>, i64> +
{

}

pub struct SystemStorageBuilder<'a, ST> {
  pub size: StorageSize,
  pub storage: &'a ST,
  pub counters: StorageSize
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


