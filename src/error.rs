// TODO 需要重构，，
#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    ParseMapsError,
    ArgsError,
    PidNotFound,
    ReadMemError(String),
    WriteMemError(String),
    #[cfg(target_os = "linux")]
    ElfError(goblin::error::Error),
    #[cfg(target_os = "windows")]
    WindowsError(windows::core::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseIntError(e) => write!(f, "Parse error: {}", e),
            Error::IoError(e) => write!(f, "Io error: {}", e),
            Error::ParseMapsError => write!(f, "Parse maps error"),
            Error::PidNotFound => write!(f, "Pid not found"),
            Error::ReadMemError(e) => write!(f, "Read mem error: {}", e),
            Error::WriteMemError(e) => write!(f, "Write mem error: {}", e),
            Error::ArgsError => write!(f, "Args error"),
            #[cfg(target_os = "linux")]
            Error::ElfError(e) => write!(f, "Elf Error: {}", e),
            #[cfg(target_os = "windows")]
            Error::WindowsError(e) => write!(f, "Windows Error: {} ", e),
        }
    }
}

#[cfg(target_os = "windows")]
impl From<windows::core::Error> for Error {
    fn from(e: windows::core::Error) -> Self {
        Error::WindowsError(e)
    }
}

#[cfg(target_os = "linux")]
impl From<goblin::error::Error> for Error {
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
