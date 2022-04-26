#![allow(unused_assignments)]

use fltk::{
    app,
    button::Button,
    enums::{Color, FrameType},
    frame::Frame,
    group::Pack,
    prelude::*,
    window::Window,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Decrement,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut num = 10000;

    // 模拟填充一些数据 不然内存数据太少，一下就找到目标了，，，
    let mut a = Vec::with_capacity(3000);
    a = (9000..10000).collect();
    a.push(0);
    let mut b = Vec::with_capacity(3000);
    b = (9000..10000).collect();
    b.push(0);
    let mut c = Vec::with_capacity(3000);
    c = (9000..10000).collect();
    c.push(0);

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default().with_size(200, 240).with_label("Click Game");
    let mut pack = Pack::default().with_size(120, 140).center_of(&wind);
    Frame::default().with_size(0, 40).with_label(
        "假装这是一个游戏，\n每点击一下 按钮 数字就会-1，\n 数字变成0的时候就会提示获胜",
    );
    pack.set_spacing(10);
    let mut frame = Frame::default()
        .with_size(0, 40)
        .with_label(&num.to_string());
    let mut but_dec = Button::default().with_size(0, 40).with_label("按钮");
    pack.end();
    wind.end();
    wind.show();

    wind.set_color(Color::White);
    but_dec.set_color(Color::from_u32(0x2962FF));
    but_dec.set_selection_color(Color::Red);
    but_dec.set_frame(FrameType::FlatBox);
    but_dec.set_label_size(20);
    but_dec.set_label_color(Color::White);
    frame.set_label_size(20);

    let (s, r) = app::channel::<Message>();

    but_dec.emit(s, Message::Decrement);

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Decrement => {
                    num = num - 1;
                    frame.set_label(&num.to_string());
                    if num < 1 {
                        frame.set_label("你赢了");
                    }
                }
            }
        }
    }
    Ok(())
}
