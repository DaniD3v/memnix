//! This module wraps the primitive rnix-ast into a more high-level format

mod lazy_eval;
mod symbol_resolver;

mod lambda;
mod let_in;

use bumpalo::Bump;
use ordered_float::NotNan;
use rnix::{
    Root,
    ast::{self, LiteralKind},
};

use crate::mir::{
    lazy_eval::Resolve,
    let_in::LetIn,
    symbol_resolver::{NullResolver, Resolver},
};

pub fn from_root_node<'bump>(root: Root, bump: &'bump Bump) -> &'bump Expr<'bump> {
    root.expr()
        .expect("parsing errors")
        .resolve(&NullResolver, bump)
}

#[derive(Debug)]
pub enum Expr<'bump> {
    Literal(Literal),
    LetIn(&'bump LetIn<'bump>),
    Param(&'bump Param),
}

#[derive(Debug)]
struct Param;

#[derive(Hash, Debug)]
pub enum Literal {
    Integer(i64),
    Float(NotNan<f64>),
    Url(),
    String(),
}

impl From<LiteralKind> for Literal {
    fn from(value: LiteralKind) -> Self {
        match value {
            rnix::ast::LiteralKind::Float(num) => {
                let num = num.value().expect("float parsing error?"); // TODO
                let num = NotNan::new(num).expect("nix floats cannot be NaN");

                Literal::Float(num)
            }
            LiteralKind::Integer(num) => Literal::Integer(num.value().expect("")),
            _ => todo!(),
        }
    }
}

impl Resolve for ast::Expr {
    type Target<'bump> = &'bump Expr<'bump>;

    fn resolve<'bump>(self, _: &impl Resolver<'bump>, bump: &'bump Bump) -> Self::Target<'bump> {
        bump.alloc(match self {
            ast::Expr::Literal(lit) => Expr::Literal(lit.kind().into()),
            ast::Expr::LetIn(let_in) => Expr::LetIn(let_in.resolve(&NullResolver, bump)),
            _ => todo!(),
        })
    }
}
