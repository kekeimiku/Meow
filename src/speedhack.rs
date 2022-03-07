#![feature(lang_items)]
//#![no_std]
#![allow(non_camel_case_types)]

//use std::simd::StdFloat;

// #[lang = "eh_personality"]
// #[no_mangle]
// pub extern "C" fn rust_eh_personality() {}

// #[panic_handler]
// #[no_mangle]
// pub extern "C" fn panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }

#[link(name = "c")]
extern "C" {
    fn __gettimeofday(tp: *mut timeval, tz: *mut timezone) -> i32;
    fn __clock_gettime(clk_id: i32, tp: *mut timespec) -> i32;
}

pub enum timezone {}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

static speedmultiplier: f64 = 0.1;

#[derive(Copy, Clone)]
pub struct init_clock {
    pub result: i32,
    pub initialoffset: timespec,
    pub initialtime: timespec,
}

pub static mut INITIALCLOCK: [init_clock; 10] = [init_clock {
    result: 0,
    initialoffset: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
    initialtime: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
}; 10];

pub static mut INITIAL_TIME_TOD_TV: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
};

pub static mut INITIAL_OFFSET_TOD_TV: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
};

#[no_mangle]
pub unsafe extern "C" fn clock_gettime(clk_id: i32, tp: *mut timespec) -> i32 {
    //当前正确的时间
    let mut currenttp = timespec {
        ..Default::default()
    };
    let r = __clock_gettime(clk_id, &mut currenttp);

    if clk_id <= 9 && INITIALCLOCK[clk_id as usize].result == 0 {
        let mut temptp = timespec {
            ..Default::default()
        };

        temptp.tv_sec = currenttp.tv_sec - INITIALCLOCK[clk_id as usize].initialtime.tv_sec;
        temptp.tv_nsec = currenttp.tv_nsec - INITIALCLOCK[clk_id as usize].initialtime.tv_nsec;

        if temptp.tv_nsec < 0 {
            temptp.tv_nsec += 1000000000;
            temptp.tv_sec -= 1;
        };

        let newsec_double: f64 = temptp.tv_sec as f64 * speedmultiplier;
        let mut newnsec: i64 = (temptp.tv_nsec as f64 * speedmultiplier).floor() as i64;
        let mut newsec: i64 = newsec_double.floor() as i64;
        newnsec = (newnsec as f64
            + (newsec_double - newsec_double.floor() * 1000000000.0f32 as f64))
            as i64;

        newsec += INITIALCLOCK[clk_id as usize].initialoffset.tv_sec;
        newnsec += INITIALCLOCK[clk_id as usize].initialoffset.tv_nsec;

        newsec += newnsec / 1000000000;
        newnsec = newnsec % 1000000000;

        if newnsec < 0 {
            newnsec += 1000000000;
            newsec -= 1;
        }

        if !tp.is_null() {
            (*tp).tv_sec = newsec;
            (*tp).tv_nsec = newnsec
        }
    } else if !tp.is_null() {
        *tp = currenttp
    }

    r
}

#[no_mangle]
pub unsafe extern "C" fn gettimeofday(tv: *mut timeval, tz: *mut timezone) -> i32 {
    let mut r: i32 = 0;
    let mut currenttv: timeval = timeval {
        ..Default::default()
    };

    r = __gettimeofday(&mut currenttv, tz);

    let mut temptv: timeval = timeval {
        ..Default::default()
    };

    temptv.tv_sec = currenttv.tv_sec - INITIAL_TIME_TOD_TV.tv_sec;
    temptv.tv_usec = currenttv.tv_usec - INITIAL_TIME_TOD_TV.tv_usec;

    if temptv.tv_usec < 0 {
        temptv.tv_usec += 1000000;
        temptv.tv_sec -= 1;
    }

    let newsec_double: f64 = temptv.tv_sec as f64 * speedmultiplier as f64;
    let mut newusec: i64 = (temptv.tv_usec as f64 * speedmultiplier as f64).floor() as i64;
    let mut newsec: i64 = newsec_double.floor() as i64;

    newusec = (newusec as f64
        + (newsec_double - newsec_double.floor() * 1000000.0f32 as f64))
        as i64;

    newsec += INITIAL_OFFSET_TOD_TV.tv_sec;
    newusec += INITIAL_OFFSET_TOD_TV.tv_usec;

    newsec += newusec / 1000000;
    newusec = newusec % 1000000;

    if newusec < 0 {
        newusec += 1000000;
        newsec -= 1;
    }

    if !tv.is_null() {
        (*tv).tv_sec = newsec;
        (*tv).tv_usec = newusec
    }

    r
}
