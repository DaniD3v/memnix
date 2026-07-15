use blake3::Hasher;

use crate::{
    ArenaId,
    eval::{
        EvalResult, EvalState,
        value::{RuntimeLambda, RuntimeValue},
    },
};

/// Invariant:
/// function inputs must be hashed after the function has executed.
pub trait EvalHash<'id> {
    fn hash(self, hasher: &mut Hasher, state: &EvalState<'id, '_>);
}

#[repr(u8)]
pub enum TypeDiscriminant {
    RuntimeLambda,

    DeferredThunk,
}

impl TypeDiscriminant {
    pub fn apply(self, hasher: &mut Hasher) {
        hasher.update(&[self as u8]);
    }
}

impl<'id> EvalHash<'id> for &EvalResult<'id> {
    fn hash(self, hasher: &mut Hasher, state: &EvalState<'id, '_>) {
        let Ok(value) = self else {
            todo!("errors cannot be memoized yet");
        };

        match value {
            RuntimeValue::Lambda(lambda) => lambda.hash(hasher, state),

            _ => todo!(),
        }
    }
}

impl<'id> EvalHash<'id> for ArenaId<'id> {
    fn hash(self, hasher: &mut Hasher, state: &EvalState<'id, '_>) {
        hasher.update(state.arena[self].color().unwrap().as_bytes());
    }
}

impl<'id> EvalHash<'id> for &RuntimeLambda<'id> {
    fn hash(self, hasher: &mut Hasher, state: &EvalState<'id, '_>) {
        TypeDiscriminant::RuntimeLambda.apply(hasher);

        // TODO: only hash captures that are actually used
        for capture in self.captures().iter() {
            capture.hash(hasher, state)
        }

        self.body().hash(hasher, state);
    }
}
