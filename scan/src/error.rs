// TODO 需要重构，，
#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    SysCall(std::io::Error),
    ParseMapsError,
    ArgsError,
    PidNotFound,
    ReadMemError,
    WriteMemError,
    MprotectError,
    ElfError(goblin::error::Error)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseIntError(e) => write!(f, "Parse error: {}", e),
            Error::IoError(e) => write!(f, "Io error: {}", e),
            Error::ParseMapsError => write!(f, "Parse maps error"),
            Error::SysCall(e) => write!(f, "Syscall Error: {}", e),
            Error::PidNotFound => write!(f, "Pid not found"),
            Error::ReadMemError => write!(f, "Read mem error"),
            Error::WriteMemError => write!(f, "Write mem error"),
            Error::MprotectError => write!(f, "Mprotect error"),
            Error::ArgsError => write!(f, "Args error"),
            Error::ElfError(e) => write!(f, "Elf Error: {}", e),
        }
    }
}

impl From<goblin::error::Error> for Error{
    fn from(e: goblin::error::Error) -> Self {
        Error::ElfError(e)
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

pub type Result<T, E = Error> = std::result::Result<T, E>;
