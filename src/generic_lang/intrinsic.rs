use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

use crate::arena::DebugWith;

macro_rules! intrinsics {
    ($($name:ident( $($label:literal),+ )),* $(,)?) => {
        #[derive(Serialize, Deserialize, Clone, Debug, strum::IntoStaticStr)]
        pub enum GenericIntrinsic<E> {
           $($name([E; __arity::$name]),)*
        }

        // HACK: use metavar expressions once stable
        // serde derive does not support arbitrary const expressions
        // they are thus put into hidden constants
        #[doc(hidden)]
        mod __arity {$(
            #[allow(non_upper_case_globals)]
            pub(super) const $name: usize = [$($label,)*].len();
        )*}

        impl<E> GenericIntrinsic<E> {
            pub fn convert_inner<To>(self, map: impl Fn(E) -> To) -> GenericIntrinsic<To> {
                match self {$(
                    Self::$name(inner) => GenericIntrinsic::$name(inner.map(map)),
                )*}
            }

            pub fn edges(&self) -> impl Iterator<Item = &E> {
                match self {$(
                    Self::$name(inner) => inner.iter(),
                )*}
            }

            pub fn edges_labeled(&self) -> impl Iterator<Item = (&E, &str)> {
                match self {$(
                    Self::$name(inner) => inner.iter().zip(
                        [$($label),*].iter().copied()
                    ),
                )*}
            }
        }
    };
}

intrinsics! {
    LambdaCall("lambda", "arg"),

    LessOrEq("l", "r"),
    Add("l", "r"),
    Subtract("l", "r"),

    IfElse("condition", "then", "else"),
}

impl<T, E: DebugWith<T>> DebugWith<T> for GenericIntrinsic<E> {
    fn fmt_with(&self, with: &T, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct(self.into());
        for (edge, label) in self.edges_labeled() {
            dbg.field(label, &edge.as_wrapper(with));
        }

        dbg.finish()
    }
}
