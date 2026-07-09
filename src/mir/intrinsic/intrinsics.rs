use strum::{EnumCount, EnumIter};

use crate::{
    arena::LazyArenaId,
    mir::{
        expr::GenericMirExpr,
        ident_resolver::Resolver,
        lang::{LazyExprArena, LazyMirLambda},
    },
};

#[derive(EnumIter, EnumCount, Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Intrinsic {
    IfElse,
    LessOrEq,
    Add,
    Subtract,

    /// An intrinsic that always errors.
    ///
    /// This is used to construct expressions as a
    /// placeholder for nodes that are nothing.
    /// e.g. let x=y; y=x; in x
    RefCycleError,
}

impl Intrinsic {
    pub fn get_lambda<'b>(self, resolver: impl Resolver<'b>) -> LazyArenaId<'b> {
        resolver.get_builtins().get(self)
    }

    pub(super) fn new_wrapped<'b>(self, bump: &mut LazyExprArena<'b>) -> LazyArenaId<'b> {
        let params = self.get_params();

        if params.is_empty() {
            bump.alloc(GenericMirExpr::Intrinsic(self))
        } else {
            LazyMirLambda::with_params(self, params, bump)
        }
    }

    /// parameter names of the function called
    /// the specific names are mainly for documentation, the count of parameters is semantically important
    const fn get_params(&self) -> &'static [&'static str] {
        match self {
            Self::IfElse => &["condition", "then_expr", "else_expr"],
            Self::LessOrEq | Self::Add | Self::Subtract => &["l", "r"],

            Self::RefCycleError => &[],
        }
    }
}
