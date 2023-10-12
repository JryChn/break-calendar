use thiserror::Error;

#[derive(Error, Debug)]
pub enum InternalError{
    #[error("data persistence")]
    DataPersistentError,
    NotFoundError,
    EventInvalidError
}