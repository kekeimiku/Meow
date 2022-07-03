pub mod error;
pub mod mem;
pub mod plugin;
pub mod region;
pub mod scan;
pub mod value;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;
