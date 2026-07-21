use std::fmt::{Debug, Display, Formatter};

use blake3::Hasher;
use serde::{Deserialize, Serialize};

use crate::{
    ArenaId,
    coloring::ColoredExpr,
    eval::{
        CacheBackend, EvalResult, EvalState,
        value::{RuntimeLambda, RuntimeNumber, RuntimeValue, Thunk, thunk::ThunkState},
    },
};

// NewType to guard against accidental comparisons to `Color`
#[derive(Serialize, Deserialize, Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct EvalHash(blake3::Hash);

impl Display for EvalHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Invariant:
/// function inputs must be hashed after the function has executed.
pub trait EvalHashable<'id, B: CacheBackend> {
    fn hash(self, hasher: &mut Hasher, state: &EvalState<'id, '_, B>);

    fn compute_hash(self, state: &EvalState<'id, '_, B>) -> EvalHash
    where
        Self: Sized,
    {
        let mut hasher = Hasher::new();
        self.hash(&mut hasher, state);

        EvalHash(hasher.finalize())
    }
}

#[repr(u8)]
pub enum TypeDiscriminant {
    RuntimeLambda,

    Integer,
    Float,
    Bool,
}

impl TypeDiscriminant {
    pub fn apply(self, hasher: &mut Hasher) {
        hasher.update(&[self as u8]);
    }
}

pub fn hash_expr_with_callstack<'id, B: CacheBackend>(
    expr: &ColoredExpr,
    state: &EvalState<'id, '_, B>,
) -> Option<EvalHash> {
    let mut hasher = Hasher::new();
    hasher.update(expr.color().unwrap().as_bytes());

    for thunk in &*state.callstack {
        // TODO consider unforced thunks too
        if !thunk.is_forced() {
            return None;
        }

        thunk.hash(&mut hasher, state);
    }

    Some(EvalHash(hasher.finalize()))
}

impl<'id, B: CacheBackend> EvalHashable<'id, B> for &EvalResult<'id> {
    fn hash(self, hasher: &mut Hasher, state: &EvalState<'id, '_, B>) {
        match self {
            Ok(value) => value.hash(hasher, state),
            Err(_) => todo!("EvalHash errors"),
        }
    }
}

impl<'id, B: CacheBackend> EvalHashable<'id, B> for &RuntimeValue<'id> {
    fn hash(self, hasher: &mut Hasher, state: &EvalState<'id, '_, B>) {
        match self {
            RuntimeValue::Lambda(lambda) => lambda.hash(hasher, state),
            RuntimeValue::Thunk(thunk) => thunk.hash(hasher, state),

            RuntimeValue::Number(number) => number.hash(hasher, state),
            RuntimeValue::Bool(value) => {
                TypeDiscriminant::Bool.apply(hasher);
                hasher.update(&[*value as u8]);
            }
        }
    }
}

impl<'id, B: CacheBackend> EvalHashable<'id, B> for ArenaId<'id> {
    fn hash(self, hasher: &mut Hasher, state: &EvalState<'id, '_, B>) {
        hasher.update(state.arena()[self].color().unwrap().as_bytes());
    }
}

impl<'id, B: CacheBackend> EvalHashable<'id, B> for &RuntimeLambda<'id> {
    fn hash(self, hasher: &mut Hasher, state: &EvalState<'id, '_, B>) {
        TypeDiscriminant::RuntimeLambda.apply(hasher);
        hasher.update(state.arena()[self.body()].color().unwrap().as_bytes());

        // TODO: only hash captures that are actually used
        //
        // captures are guaranteed to be forced here:
        //   a lambda only reaches hashing through
        //   `to_record`, which bails on a (deferred) thunk
        for capture in self.captures().iter() {
            capture.hash(hasher, state);
        }
    }
}

impl<'id, B: CacheBackend> EvalHashable<'id, B> for &Thunk<'id> {
    fn hash(self, hasher: &mut Hasher, state: &EvalState<'id, '_, B>) {
        match &*self.state().borrow() {
            // thunks are transparent: an evaluated thunk hashes as its result
            ThunkState::Forced(evaluated) => evaluated.hash(hasher, state),

            ThunkState::Evaluating => unreachable!(),
            ThunkState::Deferred { .. } => todo!(
                "hash once the callstack is optimized to only contain the params that are actually used"
            ),
        }
    }
}

impl<'id, B: CacheBackend> EvalHashable<'id, B> for &RuntimeNumber {
    fn hash(self, hasher: &mut Hasher, _: &EvalState<'id, '_, B>) {
        match self {
            RuntimeNumber::Integer(int) => {
                TypeDiscriminant::Integer.apply(hasher);
                hasher.update(&int.to_le_bytes());
            }
            RuntimeNumber::Float(float) => {
                TypeDiscriminant::Float.apply(hasher);
                // TODO: this has a few issues:
                // -0 and +0 have different hashes
                // int hash != float hash for the same value
                hasher.update(&float.into_inner().to_le_bytes());
            }
        }
    }
}
