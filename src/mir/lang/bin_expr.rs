use rnix::ast::BinOp;

use crate::mir::{
    Intrinsic,
    error::MirResolveError,
    ident_resolver::{Resolve, Resolver},
    lang::{LazyExprArena, LazyMirLambdaCall},
};

impl Resolve for BinOp {
    type Target<'a> = LazyMirLambdaCall<'a>;

    fn resolve<'b>(
        self,
        resolver: &impl Resolver<'b>,
        bump: &mut LazyExprArena<'b>,
    ) -> Result<LazyMirLambdaCall<'b>, MirResolveError> {
        let operator_kind = self.operator().unwrap();

        let lhs = self.lhs().unwrap().resolve(resolver, bump)?;
        let rhs = self.rhs().unwrap().resolve(resolver, bump)?;

        let lambda = match operator_kind {
            rnix::ast::BinOpKind::LessOrEq => Intrinsic::LessOrEq.get_lambda(resolver),
            rnix::ast::BinOpKind::Sub => Intrinsic::Subtract.get_lambda(resolver),
            rnix::ast::BinOpKind::Add => Intrinsic::Add.get_lambda(resolver),

            _ => todo!("Translate {:?} BinOp to Mir", operator_kind),
        };

        Ok(LazyMirLambdaCall::new_curried(lambda, &[lhs, rhs], bump))
    }
}
