use crate::mir::{Param, builtins::intrinsics::Intrinsic, lambda::RawLambda};

pub type BuiltinLambda = RawLambda<BuiltinLambdaBody>;

impl BuiltinLambda {
    /// Creates a Lambda wrapping an Intrinsic with the parameter names in `params`.
    pub fn with_params(intrinsic: Intrinsic, params: &[&str]) -> Self {
        Self::at_depth(intrinsic, params, 0)
    }

    fn at_depth(intrinsic: Intrinsic, params: &[&str], depth: usize) -> Self {
        assert!(!params.is_empty());

        Self {
            param: Param::at_depth(depth),
            body: if params.len() == 1 {
                BuiltinLambdaBody::Intrinsic(intrinsic)
            } else {
                BuiltinLambdaBody::Lambda(Box::new(Self::at_depth(
                    intrinsic,
                    &params[1..],
                    depth + 1,
                )))
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum BuiltinLambdaBody {
    Lambda(Box<BuiltinLambda>),
    Intrinsic(Intrinsic),
}
