use thiserror::Error;

use crate::mir::Ident;

#[derive(Error, Debug)]
pub enum MirResolveError {
    #[error("the identifier {} could not be resolved", .0.as_ref())]
    IdentUnresolvable(Ident),
}
