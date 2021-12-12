use std::{fmt, io};

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn new<D: fmt::Display>(d: D) -> Error {
        Error { message: format!("{}", d) }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::new(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
