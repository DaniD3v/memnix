use crate::mir::ident_resolver::Resolver;

#[derive(Debug)]
pub struct Param {
    /// Every param can be uniquely identified by
    /// it's nesting depth
    nesting_depth: u32,
}

impl Param {
    pub fn new<'b>(resolver: &impl Resolver<'b>) -> Self {
        Self {
            nesting_depth: resolver.get_param_nesting_depth() + 1,
        }
    }
}
