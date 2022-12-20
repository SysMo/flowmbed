use crate::core::HeapSystemStorage;

pub static mut STORAGE: Option<*const HeapSystemStorage> = None;

macro_rules! materialize_vars {
    ($self:ident, $($v: ident),+) => {      
      let storage = unsafe {
        crate::util::debug::STORAGE
      };
      $(
        #[allow(unused)]
        let $v = *$self.$v;
      )+
    };
}

pub(crate) use materialize_vars;