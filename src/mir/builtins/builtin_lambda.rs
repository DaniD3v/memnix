use crate::{
    generic_lang::GenericLambda,
    mir::{Param, builtins::intrinsics::Intrinsic},
};

pub type BuiltinLambda = GenericLambda<BuiltinLambdaBody>;

impl BuiltinLambda {
    /// Creates a Lambda wrapping an Intrinsic with the parameter names in `params`.
    pub fn with_params(intrinsic: Intrinsic, params: &[&str]) -> Self {
        Self::at_depth(intrinsic, params, 0)
    }

    fn at_depth(intrinsic: Intrinsic, params: &[&str], depth: usize) -> Self {
        assert!(!params.is_empty());

        Self::new(
            Param::at_depth(depth),
            if params.len() == 1 {
                BuiltinLambdaBody::Intrinsic(intrinsic)
            } else {
                BuiltinLambdaBody::Lambda(Box::new(Self::at_depth(
                    intrinsic,
                    &params[1..],
                    depth + 1,
                )))
            },
        )
    }
}

#[derive(Clone, Debug)]
pub enum BuiltinLambdaBody {
    Lambda(Box<BuiltinLambda>),
    Intrinsic(Intrinsic),
}
