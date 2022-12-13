use std::ops::Add;
use const_default::ConstDefault;
use const_default_derive::ConstDefault;
use super::variables::{Parameter, Input, Output, ContinuousState, DiscreteState};
use anyhow;

#[allow(dead_code)]
#[derive(Default, Debug, ConstDefault, PartialEq, Eq)]
pub struct StorageSize {
    pub r_param: usize,
    pub b_param: usize,
    pub r_state: usize,
    pub b_state: usize,
    pub r_out: usize,
    pub b_out: usize,
}

#[allow(dead_code)]
impl StorageSize {
    pub const fn new() -> StorageSize {
        StorageSize::DEFAULT
    }
}

impl Add for StorageSize {
    type Output = StorageSize;
    fn add(self, rhs: Self) -> Self::Output {
        StorageSize { 
            r_param: self.r_param + rhs.r_param,
            b_param: self.b_param + rhs.b_param,
            r_state: self.r_state + rhs.r_state,
            b_state: self.b_state + rhs.b_state,
            r_out: self.r_out + rhs.r_out,
            b_out: self.b_out + rhs.b_out,
        }
            
    }
}


pub trait StorageItemRef {
    type ItemType;
    fn id() -> usize;
}
pub trait StorageAccess<T> {
    fn get(&self, ind: usize) -> &T;
    fn set(&self, ind: usize, value: T) -> anyhow::Result<()>;
}

pub trait SystemStorage2<'a>
{
    fn sizes() -> StorageSize;
    // r_param: &'a dyn StorageAccess<f64>,
    // b_param: &'a dyn StorageAccess<bool>,
    // r_state: &'a dyn StorageAccess<f64>,
}

pub trait SystemStorage {
    fn sizes(&self) -> StorageSize;

    fn r_param_get(&self, ind: usize) -> &f64;
    fn r_param_set(&self, ind: usize, value: f64);
    
    fn b_param_get(&self, ind: usize) -> &bool;
    fn b_param_set(&self, ind: usize, value: bool);
    
    
    fn r_state_get(&self, ind: usize) -> &f64;
    fn r_state_set(&self, ind: usize, value: f64);
    fn r_state_der_get(&self, ind: usize) -> &f64;
    fn r_state_der_set(&self, ind: usize, value: f64);
    
    fn b_state_get(&self, ind: usize) -> &bool;
    fn b_state_set(&self, ind: usize, value: bool);

    fn r_out_get(&self, ind: usize) -> &f64;
    fn r_out_set(&self, ind: usize, value: f64);
    
    fn b_out_get(&self, ind: usize) -> &bool;
    fn b_out_set(&self, ind: usize, value: bool);

    fn print_params(&self) {
        print!("r_param: ");
        for i in 0..self.sizes().r_param {
            print!("{},", self.r_param_get(i));
        }
        println!("");

        print!("b_param: ");
        for i in 0..self.sizes().b_param {
            print!("{},", self.b_param_get(i));
        }
        println!("");
    }

    fn print_states_outputs(&self) {
        print!("r_state: ");
        for i in 0..self.sizes().r_state {
            print!("{} ({}),", self.r_state_get(i), self.r_state_der_get(i));
        }
        println!("");

        print!("b_state: ");
        for i in 0..self.sizes().b_state {
            print!("{},", self.b_state_get(i));
        }
        println!("");

        print!("r_out: ");
        for i in 0..self.sizes().r_out {
            print!("{},", self.r_out_get(i));
        }
        println!("");

        print!("b_out: ");
        for i in 0..self.sizes().b_out {
            print!("{},", self.b_out_get(i));
        }
        println!("");
    }

}

pub enum SystemStorageItemId {
    RealParameter(usize),
    BoolParameter(usize),
    RealState(usize),
    BoolState(usize),
    RealOutput(usize),
    BoolOutput(usize),
}

impl<'a> From<&Parameter<'a, f64>> for SystemStorageItemId {
    fn from(x: &Parameter<'a, f64>) -> Self {
        Self::RealParameter(x.id())
    }
}

impl<'a> From<&Parameter<'a, bool>> for SystemStorageItemId {
    fn from(x: &Parameter<'a, bool>) -> Self {
        Self::BoolParameter(x.id())
    }
}

impl<'a> From<&ContinuousState<'a, f64>> for SystemStorageItemId {
    fn from(x: &ContinuousState<'a, f64>) -> Self {
        Self::RealState(x.id())
    }
}

impl<'a> From<&DiscreteState<'a, bool>> for SystemStorageItemId {
    fn from(x: &DiscreteState<'a, bool>) -> Self {
        Self::BoolState(x.id())
    }
}

impl<'a> From<&Output<'a, f64>> for SystemStorageItemId {
    fn from(x: &Output<'a, f64>) -> Self {
        Self::RealOutput(x.id())
    }
}

impl<'a> From<&Output<'a, bool>> for SystemStorageItemId {
    fn from(x: &Output<'a, bool>) -> Self {
        Self::BoolOutput(x.id())
    }
}
