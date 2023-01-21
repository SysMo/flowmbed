use core::fmt::{Debug, Display};

pub trait Device {
  type Error: Debug + Display + core::marker::Sync;
}