use rnix::ast::IfElse;

use crate::mir::{ExprArena, Intrinsic, LambdaCall, Resolve, Resolver, error::MirResolveError};

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
        bump: &mut ExprArena<'bump>,
    ) -> Result<LambdaCall<'bump>, MirResolveError> {
        let condition = self.condition().unwrap().resolve(resolver, bump)?;

        let then_expr = self.body().unwrap().resolve(resolver, bump)?;
        let else_expr = self.else_body().unwrap().resolve(resolver, bump)?;

        Ok(LambdaCall::new_curried(
            Intrinsic::IfElse.get_lambda(resolver),
            &[condition, then_expr, else_expr],
            bump,
        ))
    }
}
