//! This module wraps the primitive rnix-ast into a more high-level format

use ordered_float::NotNan;
use rnix::{Root, ast::LiteralKind};

pub fn from_root_node(root: Root) -> Expr {
    root.expr().expect("parsing errors").into()
}

#[derive(Hash, Debug)]
pub enum Expr {
    Literal(Literal),
}

#[derive(Hash, Debug)]
pub enum Literal {
    Integer(u64),
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
            _ => todo!(),
        }
    }
}

impl From<rnix::ast::Expr> for Expr {
    fn from(value: rnix::ast::Expr) -> Self {
        match value {
            rnix::ast::Expr::Literal(lit) => Expr::Literal(lit.kind().into()),

            _ => todo!(),
        }
    }
}
