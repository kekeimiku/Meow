use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use core::ptr;

pub struct MyAlloc;

pub type c_int = i32;

pub type size_t = usize;

extern "C" {
    pub fn free(p: *mut c_void);
    pub fn posix_memalign(memptr: *mut *mut c_void, align: size_t, size: size_t) -> c_int;
    pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;
}

unsafe impl GlobalAlloc for MyAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut ptr = ptr::null_mut();
        let ret = posix_memalign(
            &mut ptr,
            layout.align().max(core::mem::size_of::<usize>()),
            layout.size(),
        );
        if ret == 0 {
            ptr as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = self.alloc(layout);
        if !ptr.is_null() {
            ptr::write_bytes(ptr, 0, layout.size());
        }
        ptr
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut c_void);
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        realloc(ptr as *mut c_void, new_size) as *mut u8
    }
}
