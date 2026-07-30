pub(crate) mod env;
pub mod value;

use derive_builder::Builder;

use generativity::make_guard;

use crate::api::env::{Env, EnvSettings};

/// Reusable Eval Context - Customizes various aspects of evaluation.
///
/// Settings will be introduced in future versions
#[derive(Builder, Default)]
pub struct Evaluator {
    // TODO: CacheBackend, StoreSettings, EvalPurity
}

impl Evaluator {
    pub fn with_env<O>(
        &self,
        settings: EnvSettings,
        lambda: impl for<'id> FnOnce(Env<'id>) -> O,
    ) -> O {
        make_guard!(guard);
        let env = Env::new(settings, guard);

        lambda(env)
    }
}
