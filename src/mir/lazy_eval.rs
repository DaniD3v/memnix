use std::{cell::RefCell, fmt::Debug};

use bumpalo::Bump;

use crate::mir::symbol_resolver::Resolver;

/// Ast type that can be resolved to a Mir type
pub trait Resolve: Sized {
    type Target<'a>;
    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &'bump Bump,
    ) -> Self::Target<'bump>;
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
    state: RefCell<EvalState<'bump, A>>,
}

impl<'bump, A: Resolve> LazyEval<'bump, A>
where
    A::Target<'bump>: Copy + Debug,
{
    pub fn new(ast: A) -> Self {
        Self {
            state: RefCell::new(EvalState::Ast(ast)),
        }
    }

    pub fn resolve(&self, resolver: &impl Resolver<'bump>, bump: &'bump Bump) -> A::Target<'bump> {
        let mut this = self.state.borrow_mut();

        let ast = match std::mem::replace(&mut *this, EvalState::Evaluating) {
            EvalState::Ast(ast) => ast,
            _ => todo!("some kinda failure"),
        };

        let result = ast.resolve(resolver, bump);
        *this = EvalState::Mir(result);

        result
    }
}
