use std::fmt::{Debug, Display, Formatter};

use blake3::Hasher;
use serde::{Deserialize, Serialize};

use crate::{
    coloring::{Color, ColoredExpr, ColoredExprArena},
    eval::{
        CacheBackend, EvalState, ValueResult,
        value::{CallstackRecord, RecordRepr, ValueRecord},
    },
};

/// Hash that uniquely identifies every runtime value like `1` or `{}`
#[derive(Serialize, Deserialize, Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct ValueHash(blake3::Hash);

/// Hash that represents a specific instance of the evaluation of an expression.
///
/// e.g. hash(expr: `x + y`, callstack: `[1, 2]`)
#[derive(Serialize, Deserialize, Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct EvalHash(blake3::Hash);

impl Display for ValueHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Display for EvalHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ValueHash {
    pub fn from_record(record: &ValueRecord) -> Self {
        Self(postcard_hash(record))
    }

    pub fn new_pure<'id>(expr: &ValueResult<'id>, arena: &ColoredExprArena<'id>) -> Option<Self> {
        let record = expr.to_record(arena, |child| Self::new_pure(child, arena))?;
        Some(Self::from_record(&record))
    }
}

#[derive(Serialize, Deserialize)]
struct EvaluationRecord {
    expr: Color,
    callstack: CallstackRecord,
}

impl EvalHash {
    pub fn new_pure<'id, B: CacheBackend>(
        expr: &ColoredExpr,
        state: &EvalState<'id, '_, B>,
    ) -> Option<Self> {
        let record = EvaluationRecord {
            expr: (*expr.color())?,
            callstack: state.callstack.to_record(state.arena(), |child| {
                ValueHash::new_pure(child, state.arena())
            })?,
        };

        Some(Self(postcard_hash(&record)))
    }
}

fn postcard_hash<T: Serialize>(value: &T) -> blake3::Hash {
    let mut hasher = Hasher::new();

    // this hashes byte-for-byte which is slow
    postcard::to_io(value, &mut hasher).expect("blake3::Hasher should be infallible");

    hasher.finalize()
}
