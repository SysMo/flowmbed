use core::cell::{Cell, RefCell, RefMut, UnsafeCell};
use const_default::ConstDefault;
use const_default_derive::ConstDefault;

#[derive(Default, ConstDefault)]
pub struct RefOnce<T> {
  taken: Cell<bool>,
  instance: RefCell<Option<T>>
}

impl<T> RefOnce<T> {
  pub const fn new() -> Self {
    RefOnce { taken: Cell::new(false), instance: RefCell::new(None) }
  }

  pub fn init(&self, x: T) -> anyhow::Result<()> {
    if !self.instance.borrow().is_some() {
      self.instance.replace(Some(x));
    } else {
      anyhow::bail!("Instance already initialized")
    }
    Ok(())
  }

  pub fn mut_ref<'a>(&self) -> anyhow::Result<RefMut<'a, T>> {
    if !self.taken.get() {
      if self.instance.borrow().is_some() {
        self.taken.set(true);
        // let rfm = self.instance.borrow_mut();
        let rfm = RefMut::map(
          self.instance.borrow_mut(), |x| match x {
            Some(v) => v,
            None => panic!("Unreachable code!"),
          }
        );
        let rfm = unsafe {
          std::mem::transmute::<RefMut<'_, T>, RefMut<'a, T>>(rfm)
        };
        Ok(rfm)        
      } else {
        anyhow::bail!("Value not initialized")
      }
    } else {
      anyhow::bail!("Mutable reference already created")
    }
  } 
}

unsafe impl<T> Sync for RefOnce<T> {}


pub struct OnceCell<T> {
    // Invariant: written to at most once.
    inner: UnsafeCell<Option<T>>,
}

impl<T> OnceCell<T> {
  /// Creates a new empty cell.
  // #[must_use]
  pub const fn new() -> OnceCell<T> {
      OnceCell { inner: UnsafeCell::new(None) }
  }

  pub fn get(&self) -> anyhow::Result<&T> {
      // SAFETY: Safe due to `inner`'s invariant
      unsafe { &*self.inner.get() }.as_ref().ok_or_else(
        || anyhow::anyhow!("OnceCell value not initialized")
      )
  }

  pub fn get_mut(&mut self) -> anyhow::Result<&mut T> {
    self.inner.get_mut().as_mut().ok_or_else(
      || anyhow::anyhow!("OnceCell value not initialized")
    )
  }

  pub fn set(&self, value: T) -> anyhow::Result<()> {
    // SAFETY: Safe because we cannot have overlapping mutable borrows
    let slot = unsafe { &*self.inner.get() };
    if slot.is_some() {
        anyhow::bail!("Attempt to initialize cell again!");
    }

    // SAFETY: This is the only place where we set the slot, no races
    // due to reentrancy/concurrency are possible, and we've
    // checked that slot is currently `None`, so this write
    // maintains the `inner`'s invariant.
    let slot = unsafe { &mut *self.inner.get() };
    *slot = Some(value);
    Ok(())
  }

}

impl<T> ConstDefault for OnceCell<T> {
  const DEFAULT: Self = OnceCell::new();
}

// We must know that only one thread will access a cell
unsafe impl<T> Sync for OnceCell<T> {}