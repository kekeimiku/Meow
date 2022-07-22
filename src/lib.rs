pub mod error;
pub mod mem;
// pub mod prompt;
pub mod scan;
pub mod value;

#[cfg(feature = "plugin")]
pub mod plugin;

mod os;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use os::linux as platform;

#[cfg(target_os = "windows")]
pub use os::windows as platform;

#[cfg(target_os = "macos")]
pub use os::macos as platform;
