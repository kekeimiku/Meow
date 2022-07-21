// TODO refactor

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    ParseMapsError,    
    GetLastError(u32),
    #[cfg(feature = "plugin")]
    UseExtError(libloading::Error),
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

#[cfg(feature = "plugin")]
impl From<libloading::Error> for Error {
    fn from(e: libloading::Error) -> Self {
        Error::UseExtError(e)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
