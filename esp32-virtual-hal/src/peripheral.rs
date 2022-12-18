use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct PeripheralRef<'a, T> {
  inner: T,
  _lifetime: PhantomData<&'a mut T>,
}

impl<'a, T> PeripheralRef<'a, T> {
  #[inline]
  pub fn new(inner: T) -> Self {
      Self {
          inner,
          _lifetime: PhantomData,
      }
  }

  /// Unsafely clone (duplicate) a peripheral singleton.
  ///
  /// # Safety
  ///
  /// This returns an owned clone of the peripheral. You must manually ensure
  /// only one copy of the peripheral is in use at a time. For example, don't
  /// create two SPI drivers on `SPI1`, because they will "fight" each other.
  ///
  /// You should strongly prefer using `reborrow()` instead. It returns a
  /// `PeripheralRef` that borrows `self`, which allows the borrow checker
  /// to enforce this at compile time.
  pub unsafe fn clone_unchecked(&mut self) -> PeripheralRef<'a, T>
  where
      T: Peripheral<P = T>,
  {
      PeripheralRef::new(self.inner.clone_unchecked())
  }

  /// Reborrow into a "child" PeripheralRef.
  ///
  /// `self` will stay borrowed until the child PeripheralRef is dropped.
  pub fn reborrow(&mut self) -> PeripheralRef<'_, T>
  where
      T: Peripheral<P = T>,
  {
      // safety: we're returning the clone inside a new PeripheralRef that borrows
      // self, so user code can't use both at the same time.
      PeripheralRef::new(unsafe { self.inner.clone_unchecked() })
  }
}

impl<'a, T> Deref for PeripheralRef<'a, T> {
  type Target = T;

  #[inline]
  fn deref(&self) -> &Self::Target {
      &self.inner
  }
}

impl<'a, T> DerefMut for PeripheralRef<'a, T> {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.inner
  }
}

pub trait Peripheral: Sized {
  type P;

  unsafe fn clone_unchecked(&mut self) -> Self::P;
  
  #[inline]
  fn into_ref<'a>(mut self) -> PeripheralRef<'a, Self::P>
  where
      Self: 'a,
  {
      PeripheralRef::new(unsafe { self.clone_unchecked() })
  }
}

impl<T: DerefMut> Peripheral for T
where
    T::Target: Peripheral,
{
    type P = <T::Target as Peripheral>::P;

    #[inline]
    unsafe fn clone_unchecked(&mut self) -> Self::P {
        self.deref_mut().clone_unchecked()
    }
}