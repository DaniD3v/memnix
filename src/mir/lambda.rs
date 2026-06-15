use crate::mir::{builtins::BuiltinLambda, lang::LangLambda};

#[derive(Debug)]
pub enum Lambda<'bump> {
    Lang(LangLambda<'bump>),
    Builtin(&'static BuiltinLambda),
}
