use bumpalo::Bump;
use getset::CopyGetters;
use rnix::ast;

use crate::mir::{Expr, Resolve, Resolver, error::MirResolveError};

#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
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
    ) -> Self {
        assert!(!args.is_empty());

        if args.len() == 1 {
            LambdaCall {
                lambda,
                argument: args[0],
            }
        } else {
            let (argument, curried_args) = args.split_last().expect("args cannot be empty");
            // The lambda that is to the left of this lambda.
            // e.g. `builtins.add 1` in `(builtins.add 1) 2`
            let inner = bump.alloc(Expr::LambdaCall(LambdaCall::new_curried(
                lambda,
                curried_args,
                bump,
            )));

            LambdaCall {
                lambda: inner,
                argument,
            }
        }
    }
}

impl Resolve for ast::Apply {
    type Target<'bump> = LambdaCall<'bump>;

    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &'bump Bump,
    ) -> Result<LambdaCall<'bump>, MirResolveError> {
        let lambda = self.lambda().unwrap().resolve(resolver, bump)?;
        let argument = self.argument().unwrap().resolve(resolver, bump)?;

        Ok(LambdaCall { lambda, argument })
    }
}
