use bumpalo::Bump;
use rnix::ast::IfElse;

use crate::mir::{
    Expr, error::MirResolveError, lambda_call::LambdaCall, lazy_eval::Resolve,
    symbol_resolver::Resolver,
};

impl Resolve for IfElse {
    type Target<'a> = &'a LambdaCall<'a>;

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
    ) -> Result<Self::Target<'bump>, MirResolveError> {
        let condition = self.condition().unwrap().resolve(resolver, bump)?;

        let builtins_if_else = bump.alloc(Expr::Lambda(resolver.get_intrinsics().if_else()));
        Ok(LambdaCall::new_curried(
            builtins_if_else,
            &[condition],
            bump,
        ))
    }
}
