use std::{
    cmp::Ordering,
    ops::{Add, Neg},
};

use ordered_float::NotNan;

use crate::eval::{builtins::FromRuntimeValue, error::EvalError, value::RuntimeValue};

#[derive(Clone, Eq, Debug)]
pub enum RuntimeNumber {
    Integer(i64),
    Float(NotNan<f64>),
}

impl RuntimeNumber {
    fn as_float(&self) -> NotNan<f64> {
        match self {
            Self::Float(f) => *f,
            Self::Integer(i) => NotNan::from(*i as i32), // TODO: lossy cast, truncates integers outside i32 range
        }
    }
}

impl<'b> FromRuntimeValue<'b> for RuntimeNumber {
    fn from(value: RuntimeValue<'b>) -> Result<Self, EvalError> {
        match value.eval_thunk() {
            RuntimeValue::Number(ret) => Ok(ret),
            _ => Err(EvalError::WrongType),
        }
    }
}

impl Add for RuntimeNumber {
    type Output = RuntimeNumber;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Integer(l + r),
            (l, r) => Self::Float(l.as_float() + r.as_float()),
        }
    }
}

impl Neg for RuntimeNumber {
    type Output = RuntimeNumber;

    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(int) => Self::Integer(-int),
            Self::Float(float) => Self::Float(-float),
        }
    }
}

impl PartialEq for RuntimeNumber {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => l == r,
            (l, r) => l.as_float() == r.as_float(),
        }
    }
}

impl Ord for RuntimeNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => l.cmp(r),
            (l, r) => l.as_float().cmp(&r.as_float()),
        }
    }
}
impl PartialOrd for RuntimeNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
