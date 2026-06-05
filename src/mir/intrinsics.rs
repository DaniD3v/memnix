use bumpalo::Bump;

use crate::mir::{Expr, lambda::Lambda};

/// The rnix does not implement Send + Sync on its types.
/// This makes it impossible to construct a LazyLock which holds Intrinsics.
///
/// As a "temporary" workaround, Intrinsics is a struct
pub struct Intrinsics<'bump> {
    if_else: &'bump Expr<'bump>,
    less_or_eq: &'bump Expr<'bump>,
}

impl<'b> Intrinsics<'b> {
    pub fn new(bump: &'b Bump) -> Self {
        Intrinsics {
            if_else: bump.alloc(Expr::Lambda(Lambda::intrinsic_with_params(
                &["condition", "then_call", "else_call"],
                bump,
            ))),

            // TODO: less_or_eq can be generated from `less`, `equals` and `and`
            less_or_eq: bump.alloc(Expr::Lambda(Lambda::intrinsic_with_params(
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
