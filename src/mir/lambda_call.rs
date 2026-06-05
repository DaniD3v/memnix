use bumpalo::Bump;
use rnix::ast;

use crate::mir::{Expr, error::MirResolveError, lazy_eval::Resolve};

#[derive(Debug)]
pub struct LambdaCall<'bump> {
    lambda: &'bump Expr<'bump>,
    argument: &'bump Expr<'bump>,
}

impl<'bump> LambdaCall<'bump> {
    /// In nix lambas only take one input parameter.
    /// In order to take multiple you simply return a second function
    /// that takes another parameter from the first function.
    pub fn new_curried(
        lambda: &'bump Expr<'bump>,
        args: &[&'bump Expr<'bump>],
        bump: &'bump Bump,
    ) -> &'bump Self {
        assert!(!args.is_empty());

        bump.alloc(LambdaCall {
            lambda,
            argument: if args.len() == 1 {
                args[0]
            } else {
                let [lambda, args @ ..] = args else {
                    unreachable!("args cannot be empty")
                };

                bump.alloc(Expr::LambdaCall(LambdaCall::new_curried(
                    lambda, args, bump,
                )))
            },
        })
    }
}

impl Resolve for ast::Apply {
    type Target<'bump> = &'bump LambdaCall<'bump>;

    fn resolve<'bump>(
        self,
        resolver: &impl super::symbol_resolver::Resolver<'bump>,
        bump: &'bump bumpalo::Bump,
    ) -> Result<&'bump LambdaCall<'bump>, MirResolveError> {
        let lambda = self.lambda().unwrap().resolve(resolver, bump)?;
        let argument = self.argument().unwrap().resolve(resolver, bump)?;

        Ok(bump.alloc(LambdaCall { lambda, argument }))
    }
}
