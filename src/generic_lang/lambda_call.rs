use std::fmt::Formatter;

use getset::Getters;

use crate::{arena::DebugWith, generic_lang::WithExprType};

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct GenericLambdaCall<E> {
    lambda: E,
    argument: E,
}

impl<E> GenericLambdaCall<E> {
    pub fn new(lambda: E, argument: E) -> Self {
        Self { lambda, argument }
    }
}

impl<E: Clone> GenericLambdaCall<E> {
    pub fn children(&self) -> impl Iterator<Item = (E, &str)> {
        [
            (self.lambda.clone(), "lambda"),
            (self.argument.clone(), "argument"),
        ]
        .into_iter()
    }
}

impl<'p, 'n, From: WithExprType<'p, 'n, To>, To> WithExprType<'p, 'n, GenericLambdaCall<To>>
    for GenericLambdaCall<From>
{
    type State<'s>
        = From::State<'s>
    where
        'p: 's;

    fn with_expr<'s>(self, state: Self::State<'s>) -> GenericLambdaCall<To> {
        GenericLambdaCall {
            lambda: self.lambda.with_expr(state.clone()),
            argument: self.argument.with_expr(state),
        }
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
