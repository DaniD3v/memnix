use std::fmt::{self, Debug};

use serde::{Deserialize, Serialize};

use crate::{
    arena::{Arena, ArenaId, DebugState, DebugWith},
    generic_lang::{GenericIntrinsic, GenericLambda},
    mir::{Literal, Param},
};

#[derive(Serialize, Deserialize, Clone)]
pub enum GenericMirExpr<Edge> {
    Lambda(GenericLambda<Edge>),
    Intrinsic(GenericIntrinsic<Edge>),

    Literal(Literal),
    Param(Param),
}

pub type MirExpr<'id> = GenericMirExpr<ArenaId<'id>>;
pub type MirExprArena<'id> = Arena<'id, MirExpr<'id>>;

pub type MirIntrinsic<'id> = GenericIntrinsic<ArenaId<'id>>;
pub type MirLambda<'id> = GenericLambda<ArenaId<'id>>;

impl<Edge> GenericMirExpr<Edge> {
    pub fn edges(&self) -> Box<dyn Iterator<Item = &Edge> + '_> {
        match self {
            Self::Lambda(lambda) => Box::new(lambda.edges()),
            Self::Intrinsic(intrinsic) => Box::new(intrinsic.edges()),

            _ => Box::new(std::iter::empty()),
        }
    }

    pub fn edges_labeled(&self) -> Box<dyn Iterator<Item = (&Edge, &str)> + '_> {
        match self {
            Self::Lambda(lambda) => Box::new(lambda.edges_labeled()),
            Self::Intrinsic(intrinsic) => Box::new(intrinsic.edges_labeled()),

            _ => Box::new(std::iter::empty()),
        }
    }

    pub fn convert_inner<To>(self, map: impl Fn(Edge) -> To) -> GenericMirExpr<To> {
        match self {
            Self::Lambda(inner) => GenericMirExpr::Lambda(inner.convert_inner(map)),
            Self::Intrinsic(inner) => GenericMirExpr::Intrinsic(inner.convert_inner(map)),

            Self::Literal(inner) => GenericMirExpr::Literal(inner),
            Self::Param(inner) => GenericMirExpr::Param(inner),
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
            Self::Lambda(inner) => inner.fmt_with(with, f),
            Self::Intrinsic(inner) => inner.fmt_with(with, f),

            Self::Literal(inner) => inner.fmt(f),
            Self::Param(inner) => inner.fmt(f),
        }
    }
}
