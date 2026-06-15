mod lambda;
mod lambda_call;

pub use lambda::GenericLambda;
pub use lambda_call::GenericLambdaCall;

/// Swaps the Expression of a generic lang item to the new Expression type `E`
pub trait WithExprType<E> {
    type Target<'a>;
    type State<'a>;

    fn with_expr<'a>(&self, state: &mut Self::State<'a>) -> Self::Target<'a>;
}

impl<T: WithExprType<E>, E> WithExprType<E> for &T {
    type Target<'a> = T::Target<'a>;
    type State<'a> = T::State<'a>;

    fn with_expr<'a>(&self, state: &mut Self::State<'a>) -> Self::Target<'a> {
        T::with_expr(self, state)
    }
}
