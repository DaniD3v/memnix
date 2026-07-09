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

impl<'id> Colorable<'id> for ArenaId<'id> {
    fn depend_on(self, hasher: &mut Hasher, arena: &Arena<'id, ColoredExpr<'_>>) {
        let bytes: &[u8] = match arena[self].color() {
            Some(color) => color.as_bytes(),
            None => b"none_placeholder",
        };

        hasher.update(bytes);
    }
}

impl<'id> Colorable<'id> for &MirLambdaCall<'id> {
    fn depend_on(self, hasher: &mut Hasher, arena: &Arena<'id, ColoredExpr<'_>>) {
        hasher.update(b"lambda_call");

        self.children().for_each(|(idx, label)| {
            idx.depend_on(hasher, arena);
            hasher.update(label.as_bytes());
        });
    }
}

impl<'id> Colorable<'id> for &MirLambda<'id> {
    fn depend_on(self, hasher: &mut Hasher, arena: &Arena<'id, ColoredExpr<'_>>) {
        hasher.update(b"lambda");

        self.param().clone().depend_on(hasher, arena);
        self.children().for_each(|(idx, label)| {
            idx.depend_on(hasher, arena);
            hasher.update(label.as_bytes());
        });
    }
}

impl<'id> Colorable<'id> for Literal {
    fn depend_on(self, hasher: &mut Hasher, _: &Arena<'id, ColoredExpr<'_>>) {
        hasher.update(b"literal");

        match self {
            Self::Integer(inner) => {
                hasher.update(b"integer");
                hasher.update(&inner.to_le_bytes());
            }

            _ => todo!(),
        };
    }
}

impl<'id> Colorable<'id> for Param {
    fn depend_on(self, hasher: &mut blake3::Hasher, _: &Arena<'id, ColoredExpr>) {
        hasher.update(b"param");
        hasher.update(&self.nesting_depth().to_le_bytes());
    }
}

impl<'id> Colorable<'id> for Intrinsic {
    fn depend_on(self, hasher: &mut blake3::Hasher, _: &Arena<'id, ColoredExpr>) {
        hasher.update(b"intrinsic");
        hasher.update(&[self as u8]);
    }
}
