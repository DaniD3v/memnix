// TODO name properly (at least remove the mir)
//! This module extends the Arena by wrapping `Expr`
//!
//! It allows storing `None` values or references
//! to other expressions directly in the arena.

use std::{fmt::Formatter, ops::Index};

use crate::{Arena, ArenaId, arena::DebugWith, mir::Expr};

pub struct ExprArena<'b>(Arena<'b, MaybeOrRefExpr<'b>>);
pub type ExprId<'b> = ArenaId<'b, MaybeOrRefExpr<'b>>;

pub enum MaybeOrRefExpr<'b> {
    Some(Expr<'b>),
    Ref(ExprId<'b>),
    None,
}

impl<'id> ExprArena<'id> {
    pub fn new() -> Self {
        Self(Arena::new())
    }

    pub fn alloc(&mut self, expr: Expr<'id>) -> ExprId<'id> {
        self.0.alloc(MaybeOrRefExpr::Some(expr))
    }

    pub fn get_index_from(&self, idx: usize) -> Option<ExprId<'id>> {
        self.0.get_index_from(idx)
    }

    pub fn size(&self) -> usize {
        self.0.size()
    }

    pub(super) fn alloc_raw(&mut self, val: MaybeOrRefExpr<'id>) -> ExprId<'id> {
        self.0.alloc(val)
    }

    pub(super) fn replace_none(&mut self, idx: ExprId<'id>, val: MaybeOrRefExpr<'id>) {
        let ret = self.0.replace(idx, val);
        assert!(matches!(ret, MaybeOrRefExpr::None));
    }
}

impl<'id> Index<ExprId<'id>> for ExprArena<'id> {
    type Output = Expr<'id>;

    fn index(&self, index: ExprId<'id>) -> &Self::Output {
        match &self.0[index] {
            MaybeOrRefExpr::Some(val) => val,
            MaybeOrRefExpr::Ref(id) => &self[*id],
            MaybeOrRefExpr::None => {
                unreachable!("deferred expressions should already be resolved on first access")
            }
        }
    }
}

pub struct DebugState<'id, 'a> {
    pub arena: &'a ExprArena<'id>,
    pub already_debugged: Vec<bool>,
}

impl<'id, 'a> DebugState<'id, 'a> {
    pub fn new(arena: &'a ExprArena<'id>) -> Self {
        Self {
            arena,
            already_debugged: vec![false; arena.size()],
        }
    }
}

impl<'id> DebugWith<DebugState<'id, '_>> for ExprId<'id> {
    fn fmt_with(&self, with: &mut DebugState<'id, '_>, f: &mut Formatter<'_>) -> std::fmt::Result {
        if with.already_debugged[self.idx()] {
            write!(f, "<<repeated: {}>>", self.idx())
        } else {
            with.already_debugged[self.idx()] = true;
            with.arena[*self].fmt_with(with, f)
        }
    }
}
