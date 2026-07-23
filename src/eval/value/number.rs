use std::{
    cmp::Ordering,
    ops::{Add, Neg},
};

use ordered_float::NotNan;
use serde::{Deserialize, Serialize};

use crate::eval::{
    CacheBackend, EvalState,
    error::EvalError,
    value::{FromThunk, Thunk, Value},
};

#[derive(Serialize, Deserialize, Clone, Eq, Debug)]
pub enum Number {
    Integer(i64),
    Float(NotNan<f64>),
}

impl Number {
    fn as_float(&self) -> NotNan<f64> {
        match self {
            Self::Float(f) => *f,
            Self::Integer(i) => NotNan::from(*i as i32), // TODO: lossy cast, truncates integers outside i32 range
        }
    }
}

impl<'b, B: CacheBackend> FromThunk<'b, B> for Number {
    fn from_thunk(value: Thunk<'b>, state: EvalState<'b, '_, B>) -> Result<Self, EvalError> {
        match value.force(state)? {
            Value::Number(ret) => Ok(ret),
            _ => Err(EvalError::WrongType),
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Integer(l + r),
            (l, r) => Self::Float(l.as_float() + r.as_float()),
        }
    }
}

impl Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(int) => Self::Integer(-int),
            Self::Float(float) => Self::Float(-float),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => l == r,
            (l, r) => l.as_float() == r.as_float(),
        }
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => l.cmp(r),
            (l, r) => l.as_float().cmp(&r.as_float()),
        }
    }
}
impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
