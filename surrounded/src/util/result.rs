use crate::util::error::SurroundedError;

pub type SurroundedResult<T> = Result<T, SurroundedError>;
