use std::{fmt::Debug, sync::Mutex};

use bumpalo::Bump;

use crate::mir::{error::MirResolveError, symbol_resolver::Resolver};

/// Ast type that can be resolved to a Mir type
pub trait Resolve: Sized {
    type Target<'a>;
    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &'bump Bump,
    ) -> Result<Self::Target<'bump>, MirResolveError>;
}

#[derive(Debug)]
enum EvalState<'bump, A: Resolve> {
    Ast(A),
    Mir(A::Target<'bump>),
    Evaluating,
}

#[derive(Debug)]
pub struct LazyEval<'bump, A: Resolve>
where
    A::Target<'bump>: Debug,
{
    state: Mutex<EvalState<'bump, A>>,
}

impl<'bump, A: Resolve> LazyEval<'bump, A>
where
    A::Target<'bump>: Copy + Debug,
{
    pub fn new(ast: A) -> Self {
        Self {
            state: Mutex::new(EvalState::Ast(ast)),
        }
    }

    pub fn resolve(
        &self,
        resolver: &impl Resolver<'bump>,
        bump: &'bump Bump,
    ) -> Result<A::Target<'bump>, MirResolveError> {
        let mut this = self.state.lock().expect("other thread should not panic");

        let ast = match std::mem::replace(&mut *this, EvalState::Evaluating) {
            EvalState::Ast(ast) => ast,
            _ => todo!("some kinda failure"),
        };

        let result = ast.resolve(resolver, bump)?;
        *this = EvalState::Mir(result);

        Ok(result)
    }
}
