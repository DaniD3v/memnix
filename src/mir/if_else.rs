use bumpalo::Bump;
use rnix::ast::IfElse;

use crate::mir::{
    error::MirResolveError, lambda_call::LambdaCall, lazy_eval::Resolve,
    symbol_resolver::Resolver,
};

impl Resolve for IfElse {
    type Target<'a> = LambdaCall<'a>;

    /// An if/else expression doesn't require a special mir type at all.,
    /// Its functionality can simply be represented by a builtin functioncall,
    ///
    /// Example:,
    /// if 1 == 2 then 1 else if 2 == 2 then 2 else 3,
    /// builtins.if (1 == 2) 1 (builtins.if (2 == 2) 2 3),
    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &'bump Bump,
    ) -> Result<LambdaCall<'bump>, MirResolveError> {
        let condition = self.condition().unwrap().resolve(resolver, bump)?;

        Ok(LambdaCall::new_curried(
            resolver.get_intrinsics().if_else(),
            &[condition],
            bump,
        ))
    }
}
