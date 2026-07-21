use thiserror::Error;

// TODO: cache errors too
#[derive(Error, Clone, Debug)]
pub enum EvalError {
    #[error("attempted calling something that's not a lambda")]
    NotALambda,

    // TODO slightly more descriptive errors
    #[error("called builtin with an incorrect type")]
    WrongType,

    #[error("reference cycle detected")]
    RefCycle,
}
