use std::io;
use std::io::Write;

pub trait Print {
    fn print(&mut self, text: &str) -> io::Result<()>;
    fn println(&mut self, text: &str) -> io::Result<()>;
}

pub struct Printer<W> {
    writer: W,
}

impl<W: Write> Printer<W> {
    pub fn new(writer: W) -> Self {
        Printer { writer }
    }
}

impl<W: Write> Print for Printer<W> {
    fn print(&mut self, text: &str) -> io::Result<()> {
        write!(self.writer, "{}", text)
    }

    fn println(&mut self, text: &str) -> io::Result<()> {
        writeln!(self.writer, "{}", text)
    }
}

mod test {
    use super::{Print, Printer};

    #[test]
    fn write_works() {
        let txt = "Make some noise!";
        let mut writer = Vec::new();
        let mut printer = Printer::new(&mut writer);

        let result = printer.print(txt);
        assert!(result.is_ok());

        let actual = String::from_utf8(writer.clone()).expect("not utf8");
        let expected = txt;

        assert_eq!(actual, expected);
    }
}
