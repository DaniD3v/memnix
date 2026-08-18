use std::fmt::{self, Debug};

use crate::{
    arena::{Arena, ArenaId, DebugState, DebugWith},
    generic_lang::{GenericLambda, GenericLambdaCall},
    mir::{Intrinsic, Literal, Param},
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
