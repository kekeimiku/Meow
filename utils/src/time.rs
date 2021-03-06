// 获取当前时间(年月日时分秒)

use std::{
    fmt,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Default, Debug)]
pub struct Time {
    sec: u8,
    min: u8,
    hour: u8,
    day: u8,
    month: u8,
    year: i32,
}

impl Time {
    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn min(&self) -> u8 {
        self.min
    }

    pub fn sec(&self) -> u8 {
        self.sec
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.min, self.sec
        )
    }
}

fn seconds_to_datetime(ts: i64, tm: &mut Time) {
    let leapyear = |year| -> bool { year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) };

    static MONTHS: [[i64; 12]; 2] = [
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    ];

    let mut year = 1970;

    let dayclock = ts % 86400;
    let mut dayno = ts / 86400;

    tm.sec = (dayclock % 60) as u8;
    tm.min = ((dayclock % 3600) / 60) as u8;
    tm.hour = (dayclock / 3600) as u8;

    loop {
        let yearsize = if leapyear(year) { 366 } else { 365 };
        if dayno >= yearsize {
            dayno -= yearsize;
            year += 1;
        } else {
            break;
        }
    }
    tm.year = year as i32;

    let mut mon = 0;
    while dayno >= MONTHS[if leapyear(year) { 1 } else { 0 }][mon] {
        dayno -= MONTHS[if leapyear(year) { 1 } else { 0 }][mon];
        mon += 1;
    }
    tm.month = mon as u8 + 1;
    tm.day = dayno as u8 + 1;
}

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("get time error")
        .as_secs()
}

// 获取当前时间，offset 0 默认utc
pub fn current_time(offset: i64) -> Time {
    let timestamp = i64::try_from(get_timestamp()).unwrap() + offset;
    let mut time = Time::default();
    seconds_to_datetime(timestamp, &mut time);
    time
}
