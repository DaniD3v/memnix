use rnix::ast::BinOp;

use crate::mir::{ExprArena, Intrinsic, LambdaCall, Resolve, Resolver, error::MirResolveError};

impl Resolve for BinOp {
    type Target<'a> = LambdaCall<'a>;

    fn resolve<'b>(
        self,
        resolver: &impl Resolver<'b>,
        bump: &mut ExprArena<'b>,
    ) -> Result<LambdaCall<'b>, MirResolveError> {
        let operator_kind = self.operator().unwrap();

        let lhs = self.lhs().unwrap().resolve(resolver, bump)?;
        let rhs = self.rhs().unwrap().resolve(resolver, bump)?;

        let lambda = match operator_kind {
            rnix::ast::BinOpKind::LessOrEq => Intrinsic::LessOrEq.get_lambda(resolver),
            rnix::ast::BinOpKind::Sub => Intrinsic::Subtract.get_lambda(resolver),
            rnix::ast::BinOpKind::Add => Intrinsic::Add.get_lambda(resolver),

            _ => todo!("Translate {:?} BinOp to Mir", operator_kind),
        };

        Ok(LambdaCall::new_curried(lambda, &[lhs, rhs], bump))
    }
}
