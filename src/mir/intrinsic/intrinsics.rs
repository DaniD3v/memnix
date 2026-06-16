use bumpalo::Bump;
use strum::{EnumCount, EnumIter};

use crate::mir::{Expr, Lambda, ident_resolver::Resolver};

#[derive(EnumIter, EnumCount, Copy, Clone, Debug)]
#[repr(u8)]
pub enum Intrinsic {
    IfElse,
    LessOrEq,
    Add,
    Subtract,
}

impl Intrinsic {
    pub fn get_lambda<'b>(self, resolver: impl Resolver<'b>) -> &'b Expr<'b> {
        resolver.get_builtins().get(self)
    }

    pub(super) fn new_wrapped(self, bump: &Bump) -> &Expr<'_> {
        let params = self.get_params();
        Lambda::with_params(self, params, bump)
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
