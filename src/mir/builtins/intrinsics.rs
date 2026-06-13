use std::{array, sync::LazyLock};

use strum::{EnumCount, EnumIter, IntoEnumIterator};

use crate::mir::{Expr, Lambda, builtins::BuiltinLambda};

#[derive(EnumIter, EnumCount, Copy, Clone, Debug)]
#[repr(u8)]
pub enum Intrinsic {
    IfElse,
    LessOrEq,
    Add,
    Subtract,
}

static WRAPPED_INTRINSICS: LazyLock<[BuiltinLambda; Intrinsic::COUNT]> = LazyLock::new(|| {
    let mut lambdas = Intrinsic::iter().map(|intrinsic| intrinsic.new_builtin_lambda());

    array::from_fn(|_| {
        lambdas
            .next()
            .expect("EnumCount should match number of variants")
    })
});

impl Intrinsic {
    pub fn get_lambda<'a>(self) -> Expr<'a> {
        Expr::Lambda(Lambda::Builtin(&WRAPPED_INTRINSICS[self as usize]))
    }

    fn new_builtin_lambda(self) -> BuiltinLambda {
        let params = self.get_params();
        BuiltinLambda::with_params(self, params)
    }

    /// parameter names of the function called
    /// the specific names are mainly for documentation, the count of parameters is semantically important
    fn get_params(&self) -> &'static [&'static str] {
        match self {
            Self::IfElse => &["condition", "then_expr", "else_expr"],
            Self::LessOrEq | Self::Add | Self::Subtract => &["l", "r"],
        }
    }
}
