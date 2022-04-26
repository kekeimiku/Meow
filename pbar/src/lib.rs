use std::cmp::Ordering;

pub struct ProgressBar {
    progress: usize,
    count: usize,
    length: usize,
    start_char: char,
    end_char: char,
    progress_char: char,
    tip_char: char,
    empty_char: char,
}

#[repr(C)]
#[derive(Debug, Default)]
struct WinSize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}

extern "C" {
    fn ioctl(fd: i32, request: u64, ...) -> i32;
}

fn get_termsize() -> WinSize {
    let us = WinSize::default();
    unsafe { ioctl(1, 0x5413, &us) };
    us
}

impl ProgressBar {
    pub fn new(count: usize) -> ProgressBar {
        ProgressBar {
            progress: 0,
            count,
            length: get_termsize().ws_col as usize - 20, // TODO 减去的宽度不应该固定
            start_char: '[',
            end_char: ']',
            progress_char: '=',
            tip_char: '>',
            empty_char: '-',
        }
    }

    pub fn inc(&mut self) {
        self.progress += 1;
        let num_progress = (self.progress * self.length) / self.count;
        let mut output = self.start_char.to_string();
        for i in 0..self.length {
            match i.cmp(&num_progress) {
                Ordering::Greater => output = format!("{}{}", output, self.empty_char),
                Ordering::Less => output = format!("{}{}", output, self.progress_char),
                Ordering::Equal => output = format!("{}{}", output, self.tip_char),
            }
        }
        output = format!(
            "{}{} {}/{} ",
            output, self.end_char, self.progress, self.count
        );

        print!("\r{}", output);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }
}
