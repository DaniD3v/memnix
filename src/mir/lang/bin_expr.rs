use rnix::ast::BinOp;

use crate::mir::{LambdaCall, Resolve, Resolver, error::MirResolveError};

impl Resolve for BinOp {
    type Target<'a> = LambdaCall<'a>;

    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &'bump bumpalo::Bump,
    ) -> Result<LambdaCall<'bump>, MirResolveError> {
        let operator_kind = self.operator().unwrap();
        let builtins = resolver.get_builtins();

        let lhs = self.lhs().unwrap().resolve(resolver, bump)?;
        let rhs = self.rhs().unwrap().resolve(resolver, bump)?;

        let lambda = match operator_kind {
            rnix::ast::BinOpKind::LessOrEq => builtins.less_or_eq(),
            rnix::ast::BinOpKind::Sub => builtins.subtract(),
            rnix::ast::BinOpKind::Add => builtins.add(),

            _ => todo!("Translate {:?} BinOp to Mir", operator_kind),
        };

        Ok(LambdaCall::new_curried(lambda, &[lhs, rhs], bump))
    }
}
