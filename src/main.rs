#![feature(once_cell)]

pub mod comm;
pub mod def;
pub mod gui;
pub mod maps;
pub mod mem;
pub mod sdiff;

use lince::gui::gui;

fn main() {
    gui();
}
