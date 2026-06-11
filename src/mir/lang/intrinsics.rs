use bumpalo::Bump;

use crate::mir::{Expr, Lambda};

/// The rnix does not implement Send + Sync on its types.
/// This makes it impossible to construct a LazyLock which holds Intrinsics.
///
/// As a "temporary" workaround, Intrinsics is a struct
pub struct Builtins<'bump> {
    if_else: &'bump Expr<'bump>,
    less_or_eq: &'bump Expr<'bump>,
}

#[derive(Copy, Clone, Debug)]
pub enum Intrinsic {
    IfElse,
    LessOrEq,
}

impl<'b> Builtins<'b> {
    pub fn new(bump: &'b Bump) -> Self {
        Builtins {
            if_else: bump.alloc(Expr::Lambda(Lambda::builtin_with_params(
                Intrinsic::IfElse,
                &["condition", "then_expr", "else_expr"],
                bump,
            ))),

            // TODO: less_or_eq can be generated from `less`, `equals` and `and`
            less_or_eq: bump.alloc(Expr::Lambda(Lambda::builtin_with_params(
                Intrinsic::LessOrEq,
                &["l", "r"],
                bump,
            ))),
        }
    }

    pub fn if_else(&self) -> &'b Expr<'b> {
        self.if_else
    }

    pub fn less_or_eq(&self) -> &'b Expr<'b> {
        self.less_or_eq
    }
}

// pub static IF: LazyLock<Lambda> = LazyLock::new(|| Lambda::intrinsic_with_params_leaking(&[]));
