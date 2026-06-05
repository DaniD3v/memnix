use bumpalo::Bump;
use getset::Getters;

use crate::mir::lambda::Lambda;

/// The rnix does not implement Send + Sync on its types.
/// This makes it impossible to construct a LazyLock which holds Intrinsics.
///
/// As a "temporary" workaround, Intrinsics is a struct
#[derive(Getters)]
#[getset(get = "pub")]
pub struct Intrinsics<'bump> {
    if_else: Lambda<'bump>,
}

impl<'b> Intrinsics<'b> {
    pub fn new(bump: &'b Bump) -> Self {
        Intrinsics {
            if_else: Lambda::intrinsic_with_params(&["condition", "then_call", "else_call"], bump),
        }
    }
}

// pub static IF: LazyLock<Lambda> = LazyLock::new(|| Lambda::intrinsic_with_params_leaking(&[]));
