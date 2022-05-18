#![feature(unix_chown)]

pub mod error;
pub mod ext;
pub mod prompt;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub mod maps;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub mod winmaps;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub mod macmaps;