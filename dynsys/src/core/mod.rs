pub mod variables;
pub mod system_storage;
pub mod heap_storage;
pub mod block;
pub mod system;
pub mod system_runner;
pub mod peripheral;

pub use variables::{Parameter, Input, Output, DiscreteState, create_default};
pub use block::{Block, BlockBuilder};
pub use system_storage::{
  DefaultSystemStrorage, StorageSize, SystemStorageBuilder,
  RequiresStorage
};
pub use heap_storage::HeapSystemStorage;
pub use system::{SystemStateInfo, DynamicalSystem};
pub use system_runner::{SystemRunner, FixedStepRunner, FixedStepRunSettings};
pub use peripheral::RequirePeripherals;

pub use i32 as Int;
pub use bool as  Bool;
pub use f32 as Float;
pub use String;

pub use std::cell::Ref as DynRef;
pub use std::cell::RefMut as DynRefMut;