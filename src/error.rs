use thiserror::Error;

#[derive(Debug, Error)]
pub enum PQError {
    #[error("Unable to obtain lock")]
    LockError,
}

/// Note: can this error handle generics like this?
impl <U>From<std::sync::PoisonError<U>> for PQError {
    fn from(_: std::sync::PoisonError<U>) -> Self {
        PQError::LockError
    }
}
