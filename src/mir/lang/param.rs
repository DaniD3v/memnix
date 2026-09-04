use getset::CopyGetters;
use serde::{Deserialize, Serialize};

use crate::mir::ident_resolver::Resolver;

#[derive(Serialize, Deserialize, Clone, PartialEq, CopyGetters, Debug)]
pub struct Param {
    /// Every param can be uniquely identified by
    /// it's nesting depth
    #[getset(get_copy = "pub")]
    nesting_level: usize,
}

impl Param {
    pub fn new<'b>(resolver: &impl Resolver<'b>) -> Self {
        Self {
            nesting_level: resolver.get_param_nesting_level(),
        }
    }

    pub fn at_depth(nesting_level: usize) -> Self {
        Self { nesting_level }
    }
}
