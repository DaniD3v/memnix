use rnix::ast::BinOp;

use crate::mir::{LambdaCall, Resolve, Resolver, builtins::Intrinsic, error::MirResolveError};

impl Resolve for BinOp {
    type Target<'a> = LambdaCall<'a>;

    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &'bump bumpalo::Bump,
    ) -> Result<LambdaCall<'bump>, MirResolveError> {
        let operator_kind = self.operator().unwrap();

        let lhs = self.lhs().unwrap().resolve(resolver, bump)?;
        let rhs = self.rhs().unwrap().resolve(resolver, bump)?;

        let lambda = match operator_kind {
            rnix::ast::BinOpKind::LessOrEq => Intrinsic::LessOrEq.get_lambda(),
            rnix::ast::BinOpKind::Sub => Intrinsic::Subtract.get_lambda(),
            rnix::ast::BinOpKind::Add => Intrinsic::Add.get_lambda(),

            _ => todo!("Translate {:?} BinOp to Mir", operator_kind),
        };

        Ok(LambdaCall::new_curried(
            // TODO: re-allocating built-ins all the time is pretty bad
            bump.alloc(lambda),
            &[lhs, rhs],
            bump,
        ))
    }
}
