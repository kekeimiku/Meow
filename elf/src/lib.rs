#[cfg(feature = "64")]
pub mod elf64;

#[cfg(feature = "32")]
pub mod elf32;

mod util;
