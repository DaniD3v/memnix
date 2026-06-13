use getset::Getters;

use crate::mir::{Param, builtins::BuiltinLambda, lang::LangLambda};

#[derive(Debug)]
pub enum Lambda<'bump> {
    Lang(LangLambda<'bump>),
    Builtin(&'static BuiltinLambda),
}

#[derive(Clone, Getters, Debug)]
pub struct RawLambda<E> {
    // theres goofy `{}` desugars too but lets ignore those for now
    pub(super) param: Param,
    #[getset(get = "pub")]
    pub(super) body: E,
}

impl<E: Copy> RawLambda<E> {
    pub fn depth(&self) -> usize {
        self.param.nesting_depth()
    }
}
