// TODO:
// #![warn(missing_docs)]
// #![warn(clippy::missing_errors_doc, clippy::missing_panics_doc)]

mod api;
mod arena;
mod coloring;
mod eval;
mod generic_lang;
mod mir;

pub use api::{
    Evaluator, EvaluatorBuilder,
    env::{Env, EnvSettings, EnvSettingsBuilder},
};

/// Contains various nix eval outputs.
pub mod value {
    pub use crate::api::value::{Number, Value, ValueKind};
}
