use std::fmt::{self, Debug};

use serde::{Deserialize, Serialize};

use crate::{
    arena::{Arena, ArenaId, DebugState, DebugWith},
    generic_lang::{GenericLambda, GenericLambdaCall},
    mir::{Intrinsic, Literal, Param},
};

#[derive(Serialize, Deserialize, Clone)]
pub enum GenericMirExpr<Edge> {
    LambdaCall(GenericLambdaCall<Edge>),
    Lambda(GenericLambda<Edge>),

    Literal(Literal),
    Param(Param),
    Intrinsic(Intrinsic),
}

pub type MirExpr<'id> = GenericMirExpr<ArenaId<'id>>;
pub type ExprArena<'id> = Arena<'id, MirExpr<'id>>;

pub type MirLambdaCall<'id> = GenericLambdaCall<ArenaId<'id>>;
pub type MirLambda<'id> = GenericLambda<ArenaId<'id>>;

impl<Edge> GenericMirExpr<Edge> {
    pub fn edges(&self) -> Box<dyn Iterator<Item = &Edge> + '_> {
        match self {
            Self::LambdaCall(lambda_call) => Box::new(lambda_call.edges()),
            Self::Lambda(lambda) => Box::new(lambda.edges()),

            _ => Box::new(std::iter::empty()),
        }
    }

    pub fn edges_labeled(&self) -> Box<dyn Iterator<Item = (&Edge, &str)> + '_> {
        match self {
            Self::LambdaCall(lambda_call) => Box::new(lambda_call.edges_labeled()),
            Self::Lambda(lambda) => Box::new(lambda.edges_labeled()),

            _ => Box::new(std::iter::empty()),
        }
    }
}

impl<Id: Clone> GenericMirExpr<Id> {
    pub fn convert_inner<To>(self, map: impl Fn(Id) -> To) -> GenericMirExpr<To> {
        match self {
            Self::LambdaCall(inner) => GenericMirExpr::LambdaCall(inner.convert_inner(map)),
            Self::Lambda(inner) => GenericMirExpr::Lambda(inner.convert_inner(map)),

            Self::Literal(inner) => GenericMirExpr::Literal(inner),
            Self::Param(inner) => GenericMirExpr::Param(inner),
            Self::Intrinsic(inner) => GenericMirExpr::Intrinsic(inner),
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
