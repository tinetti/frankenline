use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

// impl <S: AsRef<str>> From<S> for Error {
//     fn from(text: S) -> Self {
//         Error { message: format!("{}", text.as_ref()) }
//     }
// }

impl Error {
    pub fn new<D: fmt::Display>(d: D) -> Error {
        Error { message: format!("{}", d) }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// impl From<io::Error> for Error {
//     fn from(err: io::Error) -> Self {
//         Error { message: format!("{}", err) }
//     }
// }

