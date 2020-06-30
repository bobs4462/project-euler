use std::io::{prelude::*, Result};
use termion::{clear, cursor};

pub struct Printer<T: Write> {
    writer: T,
    row: u16,
    col: u16,
}

impl<'a, T: Write> Printer<T> {
    pub fn new(writer: T) -> Self {
        Printer {
            writer: writer,
            row: 1,
            col: 1,
        }
    }

    pub fn init(&mut self) {
        let s = format!("{}{}", clear::All, cursor::Goto(self.row, self.col));
        self.write(s.as_bytes()).unwrap();
    }
}

impl<'a, T: Write> Write for Printer<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writer.write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
