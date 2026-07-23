use crate::{
    ArenaId,
    eval::{
        CacheBackend, Eval, EvalState, ValueResult, callstack::Callstack, error::EvalError,
        value::Value,
    },
};

use getset::Getters;

use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Debug, Getters)]
pub struct Thunk<'id> {
    #[getset(get = "pub")]
    state: Rc<RefCell<ThunkState<'id>>>,
}

#[derive(Clone, Debug)]
pub enum ThunkState<'id> {
    Forced(ValueResult<'id>),

    // Placeholder to allow swapping out of the `RefCell`
    Evaluating,

    Deferred {
        expr: ArenaId<'id>,
        callstack: Callstack<'id>,
    },
}

pub trait FromThunk<'id, B: CacheBackend>: Sized {
    fn from_thunk(value: Thunk<'id>, state: EvalState<'id, '_, B>) -> Result<Self, EvalError>;
}

impl<'id> Thunk<'id> {
    pub fn new(expr: ArenaId<'id>, callstack: Callstack<'id>) -> Self {
        Self {
            state: Rc::new(RefCell::new(ThunkState::Deferred { expr, callstack })),
        }
    }

    pub fn new_forced(result: ValueResult<'id>) -> Self {
        Self {
            state: Rc::new(RefCell::new(ThunkState::Forced(result))),
        }
    }

    pub fn force<B: CacheBackend>(&self, state: EvalState<'id, '_, B>) -> ValueResult<'id> {
        if let ThunkState::Forced(value) = &*self.state.borrow() {
            return value.clone();
        }

        let ThunkState::Deferred { expr, callstack } = self.state.replace(ThunkState::Evaluating)
        else {
            unreachable!()
        };

        let res = expr
            .eval(EvalState {
                callstack,
                ctx: state.ctx,
            })
            .and_then(|value| value.eval_thunk(state));
        self.state.replace(ThunkState::Forced(res.clone()));

        res
    }

    pub fn is_forced(&self) -> bool {
        matches!(*self.state.borrow(), ThunkState::Forced(_))
    }
}

impl<'id> Value<'id> {
    pub fn eval_thunk<B: CacheBackend>(self, state: EvalState<'id, '_, B>) -> ValueResult<'id> {
        match self {
            Self::Thunk(thunk) => thunk.force(state),
            any => Ok(any),
        }
    }
}
