#![no_std]

#[cfg(all(feature = "64", target_os = "linux"))]
pub mod elf64;

#[cfg(all(feature = "32", target_os = "linux"))]
pub mod elf32;

#[cfg(all(feature = "64", target_os = "android"))]
pub mod elf64;

#[cfg(all(feature = "32", target_os = "android"))]
pub mod elf32;

mod util;
