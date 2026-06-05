use ordered_float::NotNan;
use rnix::ast::LiteralKind;

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
