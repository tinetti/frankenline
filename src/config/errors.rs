use std::fmt;

#[derive(Debug)]
pub struct Error {
    message: String,
}

pub trait MapErrWithContext<T, E, M> {
    fn map_err_with_context<C: FnOnce() -> M>(self, op: C) -> Result<T, Error>;
}

impl<T, E: fmt::Display, M: fmt::Display> MapErrWithContext<T, E, M> for Result<T, E> {
    fn map_err_with_context<C: FnOnce() -> M>(self, op: C) -> Result<T, Error> {
        self.map_err(|err| Error { message: format!("{}\n  Caused by: {}", op(), err) })
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
