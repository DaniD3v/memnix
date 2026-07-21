mod builtins;
mod call_lambda;
mod callstack;
mod error;
mod hash;
mod memoization;
mod value;

use std::collections::HashMap;

use crate::{
    Arena, ArenaId,
    coloring::{Color, ColorableRootExpr, ColoredExpr},
    eval::{
        callstack::Callstack,
        error::EvalError,
        hash::hash_expr_with_callstack,
        memoization::Cache,
        value::{RuntimeLambda, RuntimeNumber, RuntimeValue},
    },
    mir::{Literal, MirExpr, MirLambda},
};

pub use memoization::CacheBackend;

pub type EvalResult<'id> = Result<RuntimeValue<'id>, EvalError>;

pub trait Eval<'id, B: CacheBackend> {
    fn eval(self, state: EvalState<'id, '_, B>) -> EvalResult<'id>;
}

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

pub fn eval_root_expr<'id>(root: &ColorableRootExpr<'id>) -> EvalResult<'id> {
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

    root.root_node().eval(state.clone())?.eval_thunk(state)
}

impl<'id, B: CacheBackend> Eval<'id, B> for &ColoredExpr<'id> {
    fn eval(self, state: EvalState<'id, '_, B>) -> EvalResult<'id> {
        // TODO(perf):
        // pre-evaluate callstack thunks where it is known
        // through static analysis that the thunk must be evaluated
        //
        // only consider thunks in the key if they might be used
        let cache_key = hash_expr_with_callstack(self, &state);

        if let Some(cache_key) = cache_key
            && let Some(result) = state.cache().get_result(cache_key, &state)
        {
            return Ok(result);
        }

        let result = match self.expr() {
            MirExpr::Lambda(lambda) => lambda.eval(state.clone()),
            MirExpr::LambdaCall(lambda_call) => lambda_call.eval(state.clone()),
            MirExpr::Literal(literal) => literal.eval(state.clone()),

            MirExpr::Intrinsic(intrinsic) => intrinsic.eval(state.clone()),
            MirExpr::Param(param) => Ok(RuntimeValue::Thunk(
                state.callstack[param.nesting_depth()].clone(),
            )),
        }?;

        if let Some(cache_key) = cache_key {
            state.cache().store_result(cache_key, &result, &state);
        }

        Ok(result)
    }
}

impl<'id, B: CacheBackend> Eval<'id, B> for ArenaId<'id> {
    fn eval(self, state: EvalState<'id, '_, B>) -> EvalResult<'id> {
        state.arena()[self].eval(state.clone())
    }
}

impl<'b, B: CacheBackend> Eval<'b, B> for &Literal {
    fn eval(self, _: EvalState<'b, '_, B>) -> EvalResult<'b> {
        Ok(match self {
            Literal::Integer(num) => RuntimeValue::Number(RuntimeNumber::Integer(*num)),
            Literal::Float(num) => RuntimeValue::Number(RuntimeNumber::Float(*num)),

            _ => todo!(),
        })
    }
}

impl<'b, B: CacheBackend> Eval<'b, B> for &MirLambda<'b> {
    fn eval(self, state: EvalState<'b, '_, B>) -> EvalResult<'b> {
        assert!(self.depth() <= state.callstack.len());

        Ok(RuntimeValue::Lambda(RuntimeLambda::new(
            *self.body(),
            state.callstack.prefix(self.depth()),
        )))
    }
}
