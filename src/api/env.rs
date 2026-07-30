use derive_builder::Builder;
use generativity::make_guard;

use crate::{
    api::value::EmptyError,
    arena::Arena,
    coloring::{ArenaBackedGraph, ColorableRootExpr, ColoredExpr, color_graph},
    eval::eval_root_expr,
    mir::RootExpr,
    value::Value,
};

/// Customizes an [`Env`] instance.
#[derive(Builder, Default)]
pub struct EnvSettings {
    // TODO: file loading, env
}

/// Shares nix [`Value`](crate::value::Value)s accross eval invocations.
///
/// # When to use a new `Env`
///
/// An `Env` should be re-used accross multiple evaluations if you plan
/// on re-using intermediate results / storing them in a variable.
///
/// ### Example Scenarios:
///
/// `nix repl`:
/// > can store values in variables ->  
/// > The `Env` should live as long as the repl does
///
/// `nix shell`:
/// > Builds multiple packages ->  
/// > Each package should have their own `Env` as they are independent
///
/// # Resource Usage
///
/// While Runtime [`Value`](crate::value::Value)s do get cleaned up,
/// the intermediate representation of the input files does not.
///
/// Even tho the memory cost of this intermediate representation is
/// directly proportional to the source code length, loading large
/// repositories like nixpkgs can have a significant impact.
///
/// In contrast to `nixcpp`, static imports are not resolved lazily
pub struct Env<'id> {
    arena: Arena<'id, ColoredExpr<'id>>,

    #[expect(dead_code)]
    settings: EnvSettings,
}

impl<'id> Env<'id> {
    // this is not public to avoid leaking generativity into the api
    pub(crate) fn new(settings: EnvSettings, guard: generativity::Guard<'id>) -> Self {
        Self {
            arena: Arena::new(guard),
            settings,
        }
    }

    // TODO: how should raw expressions handle imports?
    /// Evaluates and forces `expr` as a nix expression.
    ///
    /// ```
    /// # use memnix::{EnvSettings, Evaluator, value::ValueKind};
    /// # Evaluator::default().with_env(EnvSettings::default(), |mut env| {
    /// let res = env.eval_raw("1 + 1").unwrap();
    /// assert!(matches!(res.kind(), ValueKind::Number(_)));
    /// #   Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub fn eval_raw(&mut self, expr: &str) -> Result<Value<'id>, EmptyError> {
        // TODO: I should not have to manually do all these steps here
        let ast = rnix::Root::parse(expr).tree();

        // TODO: error handling
        make_guard!(temp_id);
        let mir = RootExpr::new(ast, temp_id).unwrap();

        let colored_root = ColorableRootExpr::from_mir_root(&mut self.arena, mir);
        let mut colored_graph = ArenaBackedGraph::from_root_node(colored_root);
        color_graph(&mut colored_graph);

        let res = eval_root_expr(colored_graph.root_node()).unwrap();
        Ok(Value::new(res))
    }
}
