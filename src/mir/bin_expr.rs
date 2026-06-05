use rnix::ast::BinOp;

use crate::mir::{Expr, error::MirResolveError, lambda_call::LambdaCall, lazy_eval::Resolve};

impl Resolve for BinOp {
    type Target<'a> = &'a LambdaCall<'a>;

    fn resolve<'bump>(
        self,
        resolver: &impl super::symbol_resolver::Resolver<'bump>,
        bump: &'bump bumpalo::Bump,
    ) -> Result<Self::Target<'bump>, MirResolveError> {
        let operator_kind = self.operator().unwrap();
        let intrinsics = resolver.get_intrinsics();

        let lhs = self.lhs().unwrap().resolve(resolver, bump)?;
        let rhs = self.rhs().unwrap().resolve(resolver, bump)?;

        let lambda = bump.alloc(Expr::Lambda(match self.operator().unwrap() {
            rnix::ast::BinOpKind::LessOrEq => intrinsics.less_or_eq(),

            _ => todo!("Translate {:?} BinOp to Mir", operator_kind),
        }));

        Ok(LambdaCall::new_curried(lambda, &[lhs, rhs], bump))
    }
}
