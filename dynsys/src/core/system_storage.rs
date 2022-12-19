use const_default::ConstDefault;
use const_default_derive::ConstDefault;
use super::variables::{Parameter, DiscreteState, Output, Input};

#[allow(dead_code)]
#[derive(Default, Debug, ConstDefault, PartialEq, Eq, Clone)]
pub struct StorageSize {
    pub r_param: usize,
    pub b_param: usize,
    pub i_param: usize,

    pub r_dstate: usize,
    pub b_dstate: usize,
    pub i_dstate: usize,
    
    pub r_out: usize,
    pub b_out: usize,
    pub i_out: usize
}

impl StorageSize {
  pub const fn add(self, rhs: StorageSize) -> StorageSize {
    StorageSize {
      r_param: self.r_param + rhs.r_param,
      b_param: self.b_param + rhs.b_param,
      i_param: self.i_param + rhs.i_param,
  
      r_dstate: self.r_dstate + rhs.r_dstate,
      b_dstate: self.b_dstate + rhs.b_dstate,
      i_dstate: self.i_dstate + rhs.i_dstate,
      
      r_out: self.r_out + rhs.r_out,
      b_out: self.b_out + rhs.b_out,
      i_out: self.i_out + rhs.i_out
    }
  }
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
next_index_impl!(DiscreteState, f64, r_dstate);
next_index_impl!(DiscreteState, bool, b_dstate);
next_index_impl!(DiscreteState, i64, i_dstate);
next_index_impl!(Output, f64, r_out);
next_index_impl!(Output, bool, b_out);
next_index_impl!(Output, i64, i_out);

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

  // fn set_cont_state<'a, T : 'a>(&'a self, ind: usize, v: T) -> anyhow::Result<()>
  // where Self: StorageAccess<'a, ContinuousState<'a, T>, T> {
  //   self.set_value::<ContinuousState<T>, T>(ind, v)
  // }

  fn set_discrete_state<'a, T : 'a>(&'a self, ind: usize, v: T) -> anyhow::Result<()>
  where Self: StorageAccess<'a, DiscreteState<'a, T>, T> {
    self.set_value::<DiscreteState<T>, T>(ind, v)
  }

  fn set_output<'a, T : 'a>(&'a self, ind: usize, v: T) -> anyhow::Result<()>
  where Self: StorageAccess<'a, Output<'a, T>, T> {
    self.set_value::<Output<T>, T>(ind, v)
  }
}


pub trait DefaultSystemStrorage : SystemStorageFacade 
where for<'a> Self: 
  StorageAccess<'a, Parameter<'a, f64>, f64> + 
  StorageAccess<'a, Parameter<'a, bool>, bool> +
  StorageAccess<'a, Parameter<'a, i64>, i64> +
  StorageAccess<'a, DiscreteState<'a, f64>, f64> +
  StorageAccess<'a, DiscreteState<'a, bool>, bool> +
  StorageAccess<'a, DiscreteState<'a, i64>, i64> +
  StorageAccess<'a, Output<'a, f64>, f64> + 
  StorageAccess<'a, Output<'a, bool>, bool> +
  StorageAccess<'a, Output<'a, i64>, i64> +
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

  pub fn create_output<T>(&mut self, initial: T) -> Output<'a, T>
  where 
  ST: StorageAccess<'a, Output<'a, T>, T> ,
  StorageSize: NextIndex<Output<'a, T>>
  {
    let next_index = (&mut self.counters as &mut dyn NextIndex<Output<'a, T>>).next_index();
    let output = Output { id: next_index, access: self.storage };
    self.storage.set_output(output.id, initial).unwrap();
    output
  }

  pub fn create_input<T: 'a>(&mut self) -> Input<'a, T>
  where 
  ST: StorageAccess<'a, Output<'a, T>, T> ,
  StorageSize: NextIndex<Output<'a, T>>
  {
    Input { output_id: None, access: self.storage }
  }
}

pub trait VariableCreator<'a, K, T> {  
  fn create(&mut self) -> K;
}


