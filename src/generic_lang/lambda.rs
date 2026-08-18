use std::fmt::Formatter;

use getset::Getters;

use crate::{arena::DebugWith, mir::Param};

#[derive(Clone, Getters, Debug)]
#[getset(get = "pub")]
pub struct GenericLambda<E> {
    // theres goofy `{}` desugars too but lets ignore those for now
    param: Param,
    body: E,
}

impl<E> GenericLambda<E> {
    pub fn new(param: Param, body: E) -> Self {
        Self { param, body }
    }

    pub fn depth(&self) -> usize {
        self.param.nesting_depth()
    }

    pub fn convert_inner<To>(self, map: impl Fn(E) -> To) -> GenericLambda<To> {
        GenericLambda {
            param: self.param,
            body: map(self.body),
        }
    }
}

impl<E: Clone> GenericLambda<E> {
    pub fn children(&self) -> impl Iterator<Item = (E, &str)> {
        [(self.body.clone(), "body")].into_iter()
    }
}

impl<T, E: DebugWith<T>> DebugWith<T> for GenericLambda<E> {
    fn fmt_with(&self, with: &T, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lambda")
            .field("param", self.param())
            .field("body", &self.body().as_wrapper(with))
            .finish()
    }
}
