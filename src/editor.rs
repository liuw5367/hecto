use std::io::{self, stdout};

use termion::{event::Key, raw::IntoRawMode};

use crate::Terminal;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
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
        }
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }

    pub fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Char('Q') => self.should_quit = true,
            Key::Char(c) => {
                println!("{}", c);
            }
            _ => (),
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }
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
