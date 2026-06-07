use getset::CopyGetters;

use crate::mir::ident_resolver::Resolver;

#[derive(Debug, CopyGetters)]
pub struct Param {
    /// Every param can be uniquely identified by
    /// it's nesting depth
    #[getset(get_copy = "pub")]
    nesting_depth: usize,
}

impl Param {
    pub fn new<'b>(resolver: &impl Resolver<'b>) -> Self {
        Self {
            nesting_depth: resolver.get_param_nesting_depth(),
        }
    }
}
