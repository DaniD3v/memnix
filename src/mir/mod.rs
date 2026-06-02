#![allow(dead_code)]
//! This module wraps the primitive rnix-ast into a more high-level format

mod lazy_eval;
mod symbol_resolver;

mod expression;
mod lambda;
mod let_in;

use bumpalo::Bump;
use ordered_float::NotNan;
use rnix::{Root, ast::LiteralKind};

use crate::mir::{expression::Expr, lazy_eval::Resolve, symbol_resolver::NullResolver};

pub fn from_root_node<'bump>(root: Root, bump: &'bump Bump) -> &'bump Expr<'bump> {
    root.expr()
        .expect("parsing errors")
        .resolve(&NullResolver, bump)
}

#[derive(Debug)]
pub struct Param;

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
