use std::{cell::RefCell, rc::Rc};

use blake3::Hasher;

use crate::{
    ArenaId,
    eval::{
        Eval, EvalResult, EvalState, callstack::Callstack, error::EvalError, hash::EvalHash,
        value::RuntimeValue,
    },
};

#[derive(Clone, Debug)]
pub struct Thunk<'id>(Rc<RefCell<ThunkState<'id>>>);

#[derive(Clone, Debug)]
enum ThunkState<'id> {
    Evaluated(EvalResult<'id>),

    // Placeholder to allow swapping out of the `RefCell`
    Evaluating,

    Deferred {
        expr: ArenaId<'id>,
        callstack: Callstack<'id>,
    },
}

pub trait FromThunk<'id>: Sized {
    fn from_thunk(value: Thunk<'id>, state: EvalState<'id, '_>) -> Result<Self, EvalError>;
}

impl<'id> Thunk<'id> {
    pub fn new(expr: ArenaId<'id>, callstack: Callstack<'id>) -> Self {
        Self(Rc::new(RefCell::new(ThunkState::Deferred {
            expr,
            callstack,
        })))
    }

    pub fn force(&self, state: EvalState<'id, '_>) -> EvalResult<'id> {
        if let ThunkState::Evaluated(value) = &*self.0.borrow() {
            return value.clone();
        }

        let ThunkState::Deferred { expr, callstack } = self.0.replace(ThunkState::Evaluating)
        else {
            unreachable!()
        };

        let res = expr
            .eval(EvalState {
                callstack,
                arena: state.arena,
            })
            .and_then(|value| value.eval_thunk(state));
        self.0.replace(ThunkState::Evaluated(res.clone()));

        res
    }
}

impl<'id> RuntimeValue<'id> {
    pub fn eval_thunk(self, state: EvalState<'id, '_>) -> EvalResult<'id> {
        match self {
            Self::Thunk(thunk) => thunk.force(state),
            any => Ok(any),
        }
    }
}

impl<'id> EvalHash<'id> for &Thunk<'id> {
    fn hash(self, hasher: &mut Hasher, state: &EvalState<'id, '_>) {
        match &*self.0.borrow() {
            ThunkState::Evaluated(evaluated) => evaluated.hash(hasher, state),
            ThunkState::Evaluating => unreachable!(),
            ThunkState::Deferred {
                expr: _,
                callstack: _,
            } => {
                hasher.update(b"deferred_thunk");
            }
        }
    }
}
