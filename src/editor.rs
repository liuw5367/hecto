use std::io::{self, stdout};

use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct Editor {}

fn die(e: std::io::Error) {
    panic!("{}", e);
}

impl Editor {
    pub fn default() -> Self {
        Self {}
    }

    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        for key in io::stdin().keys() {
            match key {
                Err(e) => die(e),
                Ok(key) => match key {
                    Key::Ctrl('q') => break,
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?} \r", c as u8);
                        } else {
                            println!("{:?} ({})\r", c as u8, c);
                        }

                        if c == 'q' {
                            break;
                        }
                    }
                    _ => println!("{:?}\r", key),
                },
            }
        }
    }
}
