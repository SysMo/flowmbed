use super::system_storage::StorageAccess;
use std::ops::{Deref, DerefMut};

/** Parameters */
pub struct Parameter<'a, T> {
  pub id: usize,
  pub access: &'a dyn StorageAccess<'a, Parameter<'a, T>, T>
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
