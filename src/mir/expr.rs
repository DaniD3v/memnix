use std::fmt::{self, Debug};

use crate::{
    Arena, ArenaId,
    arena::{DebugState, DebugWith, LazyArenaId},
    generic_lang::{GenericLambda, GenericLambdaCall, WithExprType},
    mir::{Intrinsic, Literal, Param, lang::LazyMirExpr},
};

pub enum GenericMirExpr<Id> {
    LambdaCall(GenericLambdaCall<Id>),
    Lambda(GenericLambda<Id>),

    Literal(Literal),
    Param(Param),
    Intrinsic(Intrinsic),
}

pub type MirExpr<'id> = GenericMirExpr<ArenaId<'id>>;
pub type ExprArena<'id> = Arena<'id, MirExpr<'id>>;

pub type MirLambdaCall<'id> = GenericLambdaCall<ArenaId<'id>>;
pub type MirLambda<'id> = GenericLambda<ArenaId<'id>>;

impl<Id: Clone> GenericMirExpr<Id> {
    pub fn children(&self) -> Box<dyn Iterator<Item = (Id, &str)> + '_> {
        match self {
            Self::LambdaCall(lambda_call) => Box::new(lambda_call.children()),
            Self::Lambda(lambda) => Box::new(lambda.children()),

            _ => Box::new(std::iter::empty()),
        }
    }
}

impl<'p, 'n: 'p> WithExprType<'p, 'n, MirExpr<'n>> for LazyMirExpr<'p> {
    type State<'s>
        = &'s dyn Fn(LazyArenaId<'p>) -> ArenaId<'n>
    where
        'p: 's;

    fn with_expr<'s>(self, state: Self::State<'s>) -> MirExpr<'n> {
        match self {
            Self::LambdaCall(inner) => MirExpr::LambdaCall(inner.with_expr(state)),
            Self::Lambda(inner) => MirExpr::Lambda(inner.with_expr(state)),

            Self::Literal(inner) => MirExpr::Literal(inner),
            Self::Param(inner) => MirExpr::Param(inner),
            Self::Intrinsic(inner) => MirExpr::Intrinsic(inner),
        }
    }
}

impl<'id> DebugWith<DebugState<'id, '_, MirExpr<'id>>> for MirExpr<'id> {
    fn fmt_with(
        &self,
        with: &DebugState<'id, '_, MirExpr<'id>>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::LambdaCall(inner) => inner.fmt_with(with, f),
            Self::Lambda(inner) => inner.fmt_with(with, f),

            Self::Literal(inner) => inner.fmt(f),
            Self::Param(inner) => inner.fmt(f),
            Self::Intrinsic(inner) => inner.fmt(f),
        }
    }
}
