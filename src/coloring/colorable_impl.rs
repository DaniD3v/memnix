use blake3::Hasher;

use crate::{
    Arena, ArenaId,
    coloring::{Colorable, ColoredExpr},
    mir::{Intrinsic, Literal, MirExpr, MirLambda, MirLambdaCall, Param},
};

impl<'id> Colorable<'id> for &MirExpr<'id> {
    fn depend_on(self, hasher: &mut Hasher, arena: &Arena<'id, ColoredExpr<'_>>) {
        match self {
            MirExpr::LambdaCall(inner) => inner.depend_on(hasher, arena),
            MirExpr::Lambda(inner) => inner.depend_on(hasher, arena),

            MirExpr::Literal(inner) => inner.clone().depend_on(hasher, arena),
            MirExpr::Param(inner) => inner.clone().depend_on(hasher, arena),
            MirExpr::Intrinsic(inner) => inner.depend_on(hasher, arena),
        }
    }
}

#[repr(u8)]
enum TypeDiscriminant {
    NonePlaceholder,
    LambdaCall,
    Lambda,
    Param,
    Intrinsic,

    LiteralInteger,
}

impl TypeDiscriminant {
    fn apply(self, hasher: &mut blake3::Hasher) {
        hasher.update(&[self as u8]);
    }
}

impl<'id> Colorable<'id> for ArenaId<'id> {
    fn depend_on(self, hasher: &mut Hasher, arena: &Arena<'id, ColoredExpr<'_>>) {
        match arena[self].color() {
            Some(color) => {
                hasher.update(color.as_bytes());
            }
            None => TypeDiscriminant::NonePlaceholder.apply(hasher),
        };
    }
}

impl<'id> Colorable<'id> for &MirLambdaCall<'id> {
    fn depend_on(self, hasher: &mut Hasher, arena: &Arena<'id, ColoredExpr<'_>>) {
        TypeDiscriminant::LambdaCall.apply(hasher);

        self.children().for_each(|(idx, label)| {
            idx.depend_on(hasher, arena);
            hasher.update(label.as_bytes());
        });
    }
}

impl<'id> Colorable<'id> for &MirLambda<'id> {
    fn depend_on(self, hasher: &mut Hasher, arena: &Arena<'id, ColoredExpr<'_>>) {
        TypeDiscriminant::Lambda.apply(hasher);

        self.param().clone().depend_on(hasher, arena);
        self.children().for_each(|(idx, label)| {
            idx.depend_on(hasher, arena);
            hasher.update(label.as_bytes());
        });
    }
}

impl<'id> Colorable<'id> for Param {
    fn depend_on(self, hasher: &mut blake3::Hasher, _: &Arena<'id, ColoredExpr>) {
        TypeDiscriminant::Param.apply(hasher);
        hasher.update(&self.nesting_depth().to_le_bytes());
    }
}

impl<'id> Colorable<'id> for Intrinsic {
    fn depend_on(self, hasher: &mut blake3::Hasher, _: &Arena<'id, ColoredExpr>) {
        TypeDiscriminant::Intrinsic.apply(hasher);
        hasher.update(&[self as u8]);
    }
}

impl<'id> Colorable<'id> for Literal {
    fn depend_on(self, hasher: &mut Hasher, _: &Arena<'id, ColoredExpr<'_>>) {
        match self {
            Self::Integer(inner) => {
                TypeDiscriminant::LiteralInteger.apply(hasher);
                hasher.update(&inner.to_le_bytes());
            }

            _ => todo!(),
        };
    }
}
