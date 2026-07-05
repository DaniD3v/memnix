use blake3::Hasher;

use crate::{
    Arena, ArenaId,
    mir::{Intrinsic, Literal, MirExpr, MirLambda, MirLambdaCall, Param},
    object_hash::{Colorable, OnceHashExpr},
};

impl<'id> Colorable<'id> for &MirExpr<'id> {
    fn depend_on(self, hasher: &mut Hasher, arena: &Arena<'id, OnceHashExpr<'_>>) {
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
    fn depend_on(self, hasher: &mut Hasher, arena: &Arena<'id, OnceHashExpr<'_>>) {
        let bytes: &[u8] = match arena[self].color() {
            Some(color) => color.as_bytes(),
            None => b"none_placeholder",
        };

        hasher.update(bytes);
    }
}

impl<'id> Colorable<'id> for &MirLambdaCall<'id> {
    fn depend_on(self, hasher: &mut Hasher, arena: &Arena<'id, OnceHashExpr<'_>>) {
        hasher.update(b"lambda_call");

        self.children().for_each(|(idx, label)| {
            idx.depend_on(hasher, arena);
            hasher.update(label.as_bytes());
        });
    }
}

impl<'id> Colorable<'id> for &MirLambda<'id> {
    fn depend_on(self, hasher: &mut Hasher, arena: &Arena<'id, OnceHashExpr<'_>>) {
        hasher.update(b"lambda");

        self.children().for_each(|(idx, label)| {
            idx.depend_on(hasher, arena);
            hasher.update(label.as_bytes());
        });
    }
}

impl<'id> Colorable<'id> for Literal {
    fn depend_on(self, hasher: &mut Hasher, _: &Arena<'id, OnceHashExpr<'_>>) {
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
    fn depend_on(self, hasher: &mut blake3::Hasher, _: &Arena<'id, OnceHashExpr>) {
        hasher.update(b"param");
        hasher.update(&self.nesting_depth().to_le_bytes());
    }
}

impl<'id> Colorable<'id> for Intrinsic {
    fn depend_on(self, hasher: &mut blake3::Hasher, _: &Arena<'id, OnceHashExpr>) {
        hasher.update(b"intrinsic");
        hasher.update(&[self as u8]);
    }
}
