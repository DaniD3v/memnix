use rnix::ast::{BinOp, BinOpKind};

use crate::{
    arena::LazyArenaId,
    generic_lang::GenericIntrinsic,
    mir::{
        error::MirResolveError,
        ident_resolver::{Resolve, Resolver},
        lang::LazyExprArena,
    },
};

impl Resolve for BinOp {
    type Target<'a> = GenericIntrinsic<LazyArenaId<'a>>;

    fn resolve<'b>(
        self,
        resolver: &impl Resolver<'b>,
        bump: &mut LazyExprArena<'b>,
    ) -> Result<Self::Target<'b>, MirResolveError> {
        let operator_kind = self.operator().unwrap();

        let params = [
            self.lhs().unwrap().resolve(resolver, bump)?,
            self.rhs().unwrap().resolve(resolver, bump)?,
        ];

        Ok(match operator_kind {
            BinOpKind::LessOrEq => GenericIntrinsic::LessOrEq(params),
            BinOpKind::Sub => GenericIntrinsic::Subtract(params),
            BinOpKind::Add => GenericIntrinsic::Add(params),

            _ => todo!("Translate {:?} BinOp to Mir", operator_kind),
        })
    }
}
