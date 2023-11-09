use std::io::{self, stdout};

use termion::{event::Key, raw::IntoRawMode};

use crate::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

fn die(e: io::Error) {
    Terminal::clear_screen();
    panic!("{e}");
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal."),
            cursor_position: Position { x: 0, y: 0 },
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome = format!("Editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_add(1));

        welcome = format!("~{}{}", spaces, welcome);
        welcome.truncate(width);

        println!("{}\r", welcome);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for row in 0..height - 1 {
            Terminal::clear_current_line();

            if row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => y = y.saturating_add(1),
            Key::Left => x = x.saturating_sub(1),
            Key::Right => x = x.saturating_add(1),
            _ => (),
        }

        self.cursor_position = Position { x, y }
    }

    pub fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Char('Q') => self.should_quit = true,
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
            _ => (),
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(e) = self.refresh_screen() {
                die(e);
            }
            if self.should_quit {
                break;
            }
            if let Err(e) = self.process_keypress() {
                die(e);
            }
        }
    }
}
