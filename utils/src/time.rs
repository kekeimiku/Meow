// 获取当前时间(年月日时分秒)

use std::{
    fmt,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Default)]
pub struct Time {
    sec: i32,
    min: i32,
    hour: i32,
    day: i32,
    month: i32,
    year: i32,
}

impl Time {
    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn month(&self) -> i32 {
        self.month
    }

    pub fn day(&self) -> i32 {
        self.day
    }

    pub fn hour(&self) -> i32 {
        self.hour
    }

    pub fn min(&self) -> i32 {
        self.min
    }

    pub fn sec(&self) -> i32 {
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

    tm.sec = (dayclock % 60) as i32;
    tm.min = ((dayclock % 3600) / 60) as i32;
    tm.hour = (dayclock / 3600) as i32;

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
    tm.month = mon as i32 + 1;
    tm.day = dayno as i32 + 1;
}

// 获取当前时间，offset 0 默认utc
pub fn current_time(offset: i64) -> Time {
    let timestamp = i64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("get time error")
            .as_secs(),
    )
    .unwrap()
        + offset;

    let mut time = Time::default();
    seconds_to_datetime(timestamp, &mut time);

    time
}
