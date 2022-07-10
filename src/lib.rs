pub mod error;
pub mod mem;
pub mod plugin;
pub mod prompt;
pub mod region;
pub mod scan;
pub mod value;

mod os;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use os::linux as platform;

#[cfg(target_os = "windows")]
pub use os::windows as platform;

#[cfg(target_os = "macos")]
pub use os::macos as platform;
