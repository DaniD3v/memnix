use thiserror::Error;

// TODO: cache errors too
#[derive(Error, Clone, Debug)]
pub enum EvalError {
    // TODO slightly more descriptive errors
    #[error("called builtin with an incorrect type")]
    WrongType,

    #[error("reference cycle detected")]
    RefCycle,
}
