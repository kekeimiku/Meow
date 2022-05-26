#![feature(exclusive_range_pattern)]

pub mod error;
pub mod ext;
pub mod prompt;
pub mod utils;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;
