use super::{system_storage::StorageAccess, SystemStateInfo};
use std::ops::Deref;
use std::fmt::Display;

/** Parameters */
pub struct Parameter<'a, T> {
  pub id: usize,
  pub access: &'a dyn StorageAccess<'a, Parameter<'a, T>, T>  
}

impl<'a, T> Parameter<'a, T> {
  pub fn reset(&self, value: T) {
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

pub struct Output<'a, T> {
  pub id: usize,
  pub access: &'a dyn StorageAccess<'a, Output<'a, T>, T>
}

impl<'a, T> Output<'a, T> {
  pub fn initialize(&self, value: T) {
    self.access.set(self.id, value).unwrap();
  }
  pub fn update(&self, value: T, _ssi: &SystemStateInfo) {
    self.access.set(self.id, value).unwrap();
  }
}

pub struct Input<'a, T> {
  pub output_id: Option<usize>,
  pub access: &'a dyn StorageAccess<'a, Output<'a, T>, T>  
}

impl <'a, T> Input<'a, T> {
  pub fn connect(&mut self, output: &Output<'a, T>) -> anyhow::Result<()> {
    match self.output_id {
      Some(id) => anyhow::bail!("Input already connected to output {}", id),
      None => {
        self.output_id = Some(output.id);
        Ok(())
      }
    }    
  }
}

impl<'a, T: Copy> Deref for Input<'a, T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
      match self.output_id {
        Some(id) => self.access.get(id),
        None => panic!("Input not connected!")
      }
      
  }
}