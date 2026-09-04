use std::fmt::Formatter;

use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::arena::DebugWith;

// TODO: copy getter when `E: Copy`
#[derive(Serialize, Deserialize, Clone, Debug, Getters)]
#[getset(get = "pub")]
pub struct GenericLambdaCall<E> {
    lambda: E,
    argument: E,
}

impl<E> GenericLambdaCall<E> {
    pub fn new(lambda: E, argument: E) -> Self {
        Self { lambda, argument }
    }

    pub fn convert_inner<To>(self, map: impl Fn(E) -> To) -> GenericLambdaCall<To> {
        GenericLambdaCall {
            lambda: map(self.lambda),
            argument: map(self.argument),
        }
    }

    pub fn edges(&self) -> impl Iterator<Item = &E> {
        [&self.lambda, &self.argument].into_iter()
    }

    pub fn edges_labeled(&self) -> impl Iterator<Item = (&E, &str)> {
        self.edges().zip(["lambda", "argument"])
    }
}

impl<T, E: DebugWith<T>> DebugWith<T> for GenericLambdaCall<E> {
    fn fmt_with(&self, with: &T, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LambdaCall")
            .field("lambda", &self.lambda().as_wrapper(with))
            .field("argument", &self.argument().as_wrapper(with))
            .finish()
    }
}
