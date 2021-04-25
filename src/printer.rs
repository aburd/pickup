use std::io;
use std::io::Write;

pub trait Print {
    fn print(&mut self, text: &str) -> io::Result<()>;
    fn println(&mut self, text: &str) -> io::Result<()>;
}

pub struct Printer<W> {
    writer: W,
}

impl <W: Write> Printer<W> {
    pub fn new(writer: W) -> Self {
        Printer { writer }
    }
}