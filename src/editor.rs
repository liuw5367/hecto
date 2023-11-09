use std::io::{self, stdin, stdout, Write};

use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct Editor {
    should_quit: bool,
}

fn die(e: io::Error) {
    panic!("{}", e);
}

pub fn read_key() -> Result<Key, io::Error> {
    loop {
        if let Some(key) = stdin().lock().keys().next() {
            return key;
        }
    }
}

impl Editor {
    pub fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Char('q') => self.should_quit = true,
            Key::Char(c) => {
                println!("{}", c);
            }
            _ => (),
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        print!("\x1b[2J");
        stdout().flush()
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
