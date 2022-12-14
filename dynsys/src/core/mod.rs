pub mod variables;
pub mod system_storage;
pub mod heap_storage;
pub mod block;
pub mod system;
pub mod system_runner;
pub mod peripheral;

pub use variables::{Parameter, Input, Output, DiscreteState};
pub use block::Block;
pub use system_storage::{
  DefaultSystemStrorage, StorageSize, SystemStorageBuilder,
  RequiresStorage
};
pub use heap_storage::HeapSystemStorage;
pub use system::{SystemStateInfo, DynamicalSystem};
pub use system_runner::{SystemRunner, FixedStepRunner, FixedStepRunSettings};
pub use peripheral::RequirePeripherals;

pub type Int = i64;
pub type Bool = bool;
pub type Float = f64;
pub use String;