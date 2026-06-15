use getset::Getters;

use crate::{generic_lang::WithExprType, mir::Param};

#[derive(Clone, Getters, Debug)]
pub struct GenericLambda<E> {
    // theres goofy `{}` desugars too but lets ignore those for now
    param: Param,

    #[getset(get = "pub")]
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

impl<From: WithExprType<To>, To> WithExprType<To> for GenericLambda<From> {
    type Target<'t> = GenericLambda<From::Target<'t>>;
    type State<'t> = From::State<'t>;

    fn with_expr<'t>(&self, state: &mut Self::State<'t>) -> Self::Target<'t> {
        GenericLambda {
            param: self.param.clone(),
            body: self.body.with_expr(state),
        }
    }
}
