use core::cell::{Cell, RefCell, RefMut};
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

  pub fn mut_ref<'a>(&'a self) -> anyhow::Result<RefMut<'a, T>> {
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