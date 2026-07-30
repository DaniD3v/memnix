use thiserror::Error;

use crate::eval::value;

/// Represents a successfully evaluated nix expression.
///
/// This includes Warnings but not Errors.
pub struct Value<'id> {
    inner: value::Value<'id>,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ValueKind {
    Number(Number),

    /// A variant to represent [`Value`s](Value) that cannot be represented in [`ValueKind`] yet.
    #[doc(hidden)]
    __NotYetImplemented,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Number {
    Integer(i64),
    // TODO: Floats
}

// opaque placeholder error struct
#[derive(Error, Debug)]
#[error("Opaque PlaceholderError")]
pub struct EmptyError;

impl<'id> Value<'id> {
    pub(crate) fn new(inner: value::Value<'id>) -> Self {
        Self { inner }
    }

    pub fn kind(&self) -> ValueKind {
        match self.inner {
            value::Value::Number(value::Number::Integer(inner)) => {
                ValueKind::Number(Number::Integer(inner))
            }

            _ => ValueKind::__NotYetImplemented,
        }
    }
}
