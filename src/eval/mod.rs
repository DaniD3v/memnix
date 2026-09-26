mod builtins; // TODO(this commit): rename to intrinsics
mod callstack;
mod error;
mod hash;
mod memoization;
pub mod value;

use std::collections::HashMap;

use crate::{
    arena::{Arena, ArenaId},
    coloring::{Color, ColorableRootExpr, ColoredExpr},
    eval::{
        callstack::Callstack,
        error::EvalError,
        hash::EvalHash,
        memoization::Cache,
        value::{Lambda, Number, Value},
    },
    mir::{Literal, MirExpr, MirLambda},
};

pub use memoization::CacheBackend;

pub type ValueResult<'id> = Result<Value<'id>, EvalError>;

pub struct EvalState<'id, 'a, B: CacheBackend> {
    callstack: Callstack<'id>,
    ctx: &'a EvalCtx<'id, 'a, B>,
}

// TODO: turn this into EvalState and pass through `Callstack` separately
// EvalState has to be cloned quickly => these are shared behind one reference.
struct EvalCtx<'id, 'a, B: CacheBackend> {
    arena: &'a Arena<'id, ColoredExpr<'id>>,
    // `ArenaId`s are not stable across runs ->
    // when loading values from cache, you have to resolve colors back to `ArenaId`'s
    color_map: HashMap<Color, ArenaId<'id>>,

    cache: Cache<B>,
}

// `derive(Clone)` would require `B: Clone`
impl<B: CacheBackend> Clone for EvalState<'_, '_, B> {
    fn clone(&self) -> Self {
        Self {
            callstack: self.callstack.clone(),
            ctx: self.ctx,
        }
    }
}

// TODO: remove this once `EvalState` and `Callstack` are split
impl<'id, 'a, B: CacheBackend> EvalState<'id, 'a, B> {
    fn arena(&self) -> &'a Arena<'id, ColoredExpr<'id>> {
        self.ctx.arena
    }

    fn colors(&self) -> &'a HashMap<Color, ArenaId<'id>> {
        &self.ctx.color_map
    }

    fn cache(&self) -> &'a Cache<B> {
        &self.ctx.cache
    }
}

pub fn eval_root_expr<'id>(root: &ColorableRootExpr<'id, '_>) -> ValueResult<'id> {
    let ctx = EvalCtx {
        arena: root.arena(),
        color_map: root
            .arena()
            .iter_indices()
            .filter_map(|id| root.arena()[id].color().map(|color| (color, id)))
            .collect(),

        cache: Cache::new(),
    };

    let state = EvalState {
        callstack: Callstack::default(),
        ctx: &ctx,
    };

    root.root_node_id().eval(state.clone())?.eval_thunk(state)
}

impl<'id> ArenaId<'id> {
    fn eval<B: CacheBackend>(self, state: EvalState<'id, '_, B>) -> ValueResult<'id> {
        let colored_expr = &state.arena()[self];

        // TODO(perf):
        // pre-evaluate callstack thunks where it is known
        // through static analysis that the thunk must be evaluated
        //
        // only consider thunks in the key if they might be used
        let cache_key = EvalHash::new_pure(colored_expr, &state);

        if let Some(cache_key) = cache_key
            && let Some(result) = state.cache().get_result(cache_key, &state)
        {
            return Ok(result);
        }

        let result = match colored_expr.expr() {
            MirExpr::Lambda(lambda) => lambda.eval(self, state.clone()),
            MirExpr::Intrinsic(intrinsic) => intrinsic.eval(state.clone()),

            MirExpr::Literal(literal) => literal.eval(),
            MirExpr::Param(param) => Ok(Value::Thunk(state.callstack[param.clone()].clone())),
        };

        if let Some(cache_key) = cache_key {
            state.cache().store_result(cache_key, &result, &state);
        }

        result
    }
}

impl Literal {
    fn eval<'b>(&self) -> ValueResult<'b> {
        Ok(match self {
            Literal::Integer(num) => Value::Number(Number::Integer(*num)),
            Literal::Float(num) => Value::Number(Number::Float(*num)),

            Literal::RefCycleError => Err(EvalError::RefCycle)?,
            _ => todo!(),
        })
    }
}

impl<'b> MirLambda<'b> {
    fn eval<B: CacheBackend>(
        &self,
        self_id: ArenaId<'b>,
        state: EvalState<'b, '_, B>,
    ) -> ValueResult<'b> {
        Ok(Value::Lambda(Lambda::new(self_id, state.callstack)))
    }
}
