mod lambda;
mod lambda_call;

pub use lambda::GenericLambda;
pub use lambda_call::GenericLambdaCall;

/// Swaps the Expression of a generic lang item to the new Expression type `E`
///
/// `'p`: lifetime of the previous expr
/// `'n`: lifetime of the next expr
pub trait WithExprType<'p, 'n, E> {
    type State<'s>: Clone
    where
        'p: 's;

    fn with_expr<'s>(&self, state: Self::State<'s>) -> E;
}

impl<'p, 'n, T: WithExprType<'p, 'n, E>, E> WithExprType<'p, 'n, E> for &T {
    type State<'s>
        = T::State<'s>
    where
        'p: 's;

    fn with_expr<'s>(&self, state: Self::State<'s>) -> E {
        T::with_expr(self, state)
    }
}
