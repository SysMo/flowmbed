use crate::core::HeapSystemStorage;

pub static mut STORAGE: Option<*const HeapSystemStorage> = None;

#[allow(unused_macros)]
macro_rules! materialize_vars {
    ($self:ident, $($v: ident),+) => {      
      let storage = unsafe {
        crate::util::debug::STORAGE
      };
      $(
        let $v = *$self.$v;
      )+
    };
}

#[allow(unused_imports)]
pub(crate) use materialize_vars;