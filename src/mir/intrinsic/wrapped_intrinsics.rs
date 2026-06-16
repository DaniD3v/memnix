use bumpalo::Bump;
use strum::{EnumCount, IntoEnumIterator};

use crate::mir::{Expr, Intrinsic};

pub struct WrappedIntrinsics<'b>([&'b Expr<'b>; Intrinsic::COUNT]);

impl<'b> WrappedIntrinsics<'b> {
    pub fn new(bump: &'b Bump) -> Self {
        let mut variants = Intrinsic::iter();
        Self(std::array::from_fn(|_| {
            variants
                .next()
                .expect("EnumCount should match number of variants")
                .new_wrapped(bump)
        }))
    }

    pub(super) fn get(&self, intrinsic: Intrinsic) -> &'b Expr<'b> {
        self.0[intrinsic as usize]
    }
}
