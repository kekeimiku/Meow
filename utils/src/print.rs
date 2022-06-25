use core::convert::TryFrom;
use std::os::raw::{c_int, c_void};

pub struct Writer(i32);

extern "C" {
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
}

impl core::fmt::Write for Writer {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        println(self.0, s)
    }
}

impl Writer {
    #[inline]
    pub fn new(handle: i32) -> Writer {
        Writer(handle)
    }

    #[inline]
    pub fn write_fmt(&mut self, args: core::fmt::Arguments) -> core::fmt::Result {
        core::fmt::Write::write_fmt(self, args)
    }

    #[inline]
    pub fn write_nl(&mut self) -> core::fmt::Result {
        println(self.0, "\n")
    }
}

#[inline]
pub fn println(handle: i32, msg: &str) -> core::fmt::Result {
    let msg = msg.as_bytes();

    let mut written = 0;
    while written < msg.len() {
        match unsafe { libc_write(handle, &msg[written..]) } {
            None | Some(0) => break,
            Some(res) => written += res,
        }
    }

    Ok(())
}

unsafe fn libc_write(handle: i32, bytes: &[u8]) -> Option<usize> {
    usize::try_from(write(handle, bytes.as_ptr().cast::<core::ffi::c_void>(), bytes.len())).ok()
}

#[macro_export]
macro_rules! libc_println {
    () => { $crate::libc_println!("") };
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut t = utils::print::Writer::new(1);
            t.write_fmt(format_args!($($arg)*));
            t.write_nl();
        }
    };
}
