use std::{ops::Deref, rc::Rc};

use crate::eval::value::Thunk;

#[derive(Clone, Default, Debug)]
pub struct Callstack<'id>(Rc<[Thunk<'id>]>);

impl<'id> Callstack<'id> {
    /// The first `depth` entries, i.e. the params at nesting depths `0..depth`.
    pub fn prefix(&self, depth: usize) -> Self {
        if depth == self.0.len() {
            return self.clone();
        }

        Self(self.0[..depth].iter().cloned().collect())
    }

    pub fn with_pushed(&self, arg: Thunk<'id>) -> Self {
        Self(self.0.iter().cloned().chain([arg]).collect())
    }

    pub fn from_thunks(thunks: Vec<Thunk<'id>>) -> Self {
        Self(Rc::from(thunks))
    }
}

impl<'id> Deref for Callstack<'id> {
    type Target = [Thunk<'id>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
