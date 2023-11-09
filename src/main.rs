#![warn(clippy::all, clippy::pedantic)]

mod editor;
mod terminal;
use editor::Editor;
pub use editor::Position;
pub use terminal::Terminal;

// fn to_ctrl_byte(c: char) -> u8 {
//     let byte = c as u8;
//     byte & 0b0001_1111
// }

fn main() {
    println!("started !");

    Editor::default().run();
}
