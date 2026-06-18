use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
};

pub trait DebugWith<T>: Sized {
    fn fmt_with(&self, with: &mut T, f: &mut Formatter<'_>) -> std::fmt::Result;

    fn as_wrapper<'a>(&'a self, with: &'a mut T) -> DebugWithWrapper<'a, T, Self> {
        DebugWithWrapper {
            inner: self,
            with: RefCell::new(with),
        }
    }
}

pub struct DebugWithWrapper<'a, W, T: DebugWith<W>> {
    inner: &'a T,
    with: RefCell<&'a mut W>,
}

impl<'a, W, T: DebugWith<W>> Debug for DebugWithWrapper<'a, W, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self { inner, with } = self;
        let with = &mut with.borrow_mut();

        inner.fmt_with(with, f)
    }
}
