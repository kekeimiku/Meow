#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    ParseMapsError,
    UseExtError(libloading::Error),
    New(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseIntError(e) => write!(f, "Parse int error: {}", e),
            Error::IoError(e) => write!(f, "Io error: {}", e),
            Error::ParseMapsError => write!(f, "failed to parse maps"),
            Error::UseExtError(e) => write!(f, "libloading error: {}", e),
            Error::New(e) => write!(f, "{}", e),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseIntError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<libloading::Error> for Error {
    fn from(e: libloading::Error) -> Self {
        Error::UseExtError(e)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
