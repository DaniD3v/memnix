use getset::Getters;

use crate::generic_lang::WithExprType;

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

impl<From: WithExprType<To>, To> WithExprType<To> for GenericLambdaCall<From>
where
    From: WithExprType<To>,
{
    type Target<'t> = GenericLambdaCall<From::Target<'t>>;
    type State<'t> = From::State<'t>;

    fn with_expr<'t>(&self, state: &mut Self::State<'t>) -> Self::Target<'t> {
        GenericLambdaCall {
            lambda: self.lambda.with_expr(state),
            argument: self.argument.with_expr(state),
        }
    }
}
