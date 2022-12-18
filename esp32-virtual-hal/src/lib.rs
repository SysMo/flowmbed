pub mod peripheral;
pub mod gpio;
pub mod peripherals;

#[allow(unused_macros)]
macro_rules! into_ref {
    ($($name:ident),*) => {
        $(
            let $name = $name.into_ref();
        )*
    }
}

#[allow(unused_macros)]
macro_rules! impl_peripheral_trait {
    ($type:ident) => {
        unsafe impl Send for $type {}

        // impl $crate::peripheral::sealed::Sealed for $type {}

        impl $crate::peripheral::Peripheral for $type {
            type P = $type;

            #[inline]
            unsafe fn clone_unchecked(&mut self) -> Self::P {
                $type { ..*self }
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! impl_peripheral {
    ($type:ident) => {
        pub struct $type(::core::marker::PhantomData<*const ()>);

        impl $type {
            /// # Safety
            ///
            /// Care should be taken not to instantiate this peripheral instance, if it is already instantiated and used elsewhere
            #[inline(always)]
            pub unsafe fn new() -> Self {
                $type(::core::marker::PhantomData)
            }
        }

        $crate::impl_peripheral_trait!($type);
    };
}

#[allow(unused_imports)]
pub(crate) use into_ref;
#[allow(unused_imports)]
pub(crate) use impl_peripheral_trait;
#[allow(unused_imports)]
pub(crate) use impl_peripheral;