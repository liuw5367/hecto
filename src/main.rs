mod editor;
use editor::Editor;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn main() {
    println!("Hello, world!");

    let editor = Editor::default();
    editor.run();
}
