use strum::{EnumCount, IntoEnumIterator};

use crate::{
    arena::LazyArenaId,
    mir::{Intrinsic, lang::LazyExprArena},
};

pub struct WrappedIntrinsics<'b>([LazyArenaId<'b>; Intrinsic::COUNT]);

impl<'b> WrappedIntrinsics<'b> {
    pub fn new(bump: &mut LazyExprArena<'b>) -> Self {
        let mut variants = Intrinsic::iter();
        Self(std::array::from_fn(|_| {
            variants
                .next()
                .expect("EnumCount should match number of variants")
                .new_wrapped(bump)
        }))
    }

    pub(super) fn get(&self, intrinsic: Intrinsic) -> LazyArenaId<'b> {
        self.0[intrinsic as usize]
    }
}
