use super::{system_storage::StorageAccess, SystemStateInfo};
use crate::util::containers::OnceCell;
use std::ops::Deref;
use std::fmt::Display;

/** Parameters */
pub struct Parameter<'a, T> {
  pub id: usize,
  pub access: &'a dyn StorageAccess<'a, Parameter<'a, T>, T>  
}

impl<'a, T> Parameter<'a, T> {
  pub fn reset(&mut self, value: T) {
    self.access.set(self.id, value).unwrap();
  }  
}


impl<'a, T: Copy> Deref for Parameter<'a, T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
      self.access.get(self.id)
  }

}

pub struct ContinuousState<'a, T> {
  pub id: usize,
  pub access: &'a dyn StorageAccess<'a, ContinuousState<'a, T>, T>
}

pub struct ContinuousStateDerivative<'a, T> {
  pub id: usize,
  pub access: &'a dyn StorageAccess<'a, ContinuousStateDerivative<'a, T>, T>
}

pub struct DiscreteState<'a, T> {
  pub id: usize,
  pub access: &'a dyn StorageAccess<'a, DiscreteState<'a, T>, T>
}

impl<'a, T: Display> DiscreteState<'a, T> {
  pub fn initialize(&self, value: T) {
    self.access.set(self.id, value).unwrap();
  }

  pub fn update(&self, value: T, _ssi: &SystemStateInfo) {
    // println!("Updated discrete state {} at t = {:.3} from {} to {}", self.id, ssi.t, self.access.get(self.id), value);
    self.access.set(self.id, value).unwrap();
  }
}

impl<'a, T: Copy + Display> Deref for DiscreteState<'a, T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
      self.access.get(self.id)
  }
}

// pub struct Output<'a, T> {
//   pub id: usize,
//   pub access: &'a dyn StorageAccess<'a, Output<'a, T>, T>
// }

// impl<'a, T> Output<'a, T> {
//   pub fn initialize(&self, value: T) {
//     self.access.set(self.id, value).unwrap();
//   }
//   pub fn update(&self, value: T, _ssi: &SystemStateInfo) {
//     self.access.set(self.id, value).unwrap();
//   }
// }

// pub struct Input<'a, T> {
//   pub output_id: Option<usize>,
//   pub access: &'a dyn StorageAccess<'a, Output<'a, T>, T>  
// }

// impl <'a, T> Input<'a, T> {
//   pub fn connect(&mut self, output: &Output<'a, T>) -> anyhow::Result<()> {
//     match self.output_id {
//       Some(id) => anyhow::bail!("Input already connected to output {}", id),
//       None => {
//         self.output_id = Some(output.id);
//         Ok(())
//       }
//     }    
//   }
// }

// impl<'a, T: Copy> Deref for Input<'a, T> {
//   type Target = T;
//   fn deref(&self) -> &Self::Target {
//       match self.output_id {
//         Some(id) => self.access.get(id),
//         None => panic!("Input not connected!")
//       }
      
//   }
// }

pub trait InitInto<T> {
  fn init_into(&self) -> T;
}

impl<T: Copy> InitInto<T> for T {
  fn init_into(&self) -> T {
    *self
  }
}

impl<T: Copy, const N: usize> InitInto<[T; N]> for T {
  fn init_into(&self) -> [T; N] {
    [*self; N]
  }
}


pub fn create_default<U: Default>() -> U {
  Default::default()
}


pub struct Output<T: Copy> {
  pub value: T
}

impl<T: Copy> Output<T> {
  pub fn new<V: InitInto<T>>(value: V) -> Self {
    Self {
      value: value.init_into()
    }
  }

  pub fn initialize(&mut self, value: T) {
    self.value = value;
  }

  pub fn update(&mut self, value: T, _ssi: &SystemStateInfo) {
    self.value = value;
  }
}

pub struct Input<'a, T: Copy> {
  output_ref: OnceCell<&'a Output<T>>
}

impl<'a, T: Copy> Input<'a, T> {
  pub fn new() -> Self {
    Self { output_ref: OnceCell::new() }
  }

  pub fn connect(&self, output: &Output<T>) -> anyhow::Result<()> {
    let out_ref_1: *const Output<T> = output;
    let out_ref_2: &'a Output<T> = unsafe { &*out_ref_1};
    self.output_ref.set(out_ref_2)
    // match &self.output_ref {
    //     Some(_) => anyhow::bail!("Input already connected"),
    //     None => self.output_ref = Some(output)
    // }
    // Ok(())
  }
}

impl<'a, T: Copy> Deref for Input<'a, T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
      match self.output_ref.get() {
        Ok(output) => &output.value,
        Err(_) => panic!("Input not connected!")
      }
      
  }
}