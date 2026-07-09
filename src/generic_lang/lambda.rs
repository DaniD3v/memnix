use std::fmt::Formatter;

use getset::Getters;

use crate::{arena::DebugWith, generic_lang::WithExprType, mir::Param};

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
}

impl<E: Clone> GenericLambda<E> {
    pub fn children(&self) -> impl Iterator<Item = (E, &str)> {
        [(self.body.clone(), "body")].into_iter()
    }
}

impl<'p, 'n, From: WithExprType<'p, 'n, To>, To> WithExprType<'p, 'n, GenericLambda<To>>
    for GenericLambda<From>
{
    type State<'s>
        = From::State<'s>
    where
        'p: 's;

    fn with_expr<'s>(self, state: Self::State<'s>) -> GenericLambda<To> {
        GenericLambda {
            param: self.param.clone(),
            body: self.body.with_expr(state),
        }
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
