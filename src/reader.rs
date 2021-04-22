use std::io::{self, BufRead};

pub trait ReadInput {
    fn read_input(&mut self) -> io::Result<String>;
}

pub struct Reader<R> {
  reader: R,
}

impl <R> Reader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: BufRead> ReadInput for Reader<R> {
  fn read_input(&mut self) -> io::Result<String> {
      let mut buf = String::new();
      self.reader.read_line(&mut buf)?;
      Ok(buf.trim().to_string())
  }
}



#[cfg(test)]
mod tests {
    use crate::reader::{Reader, ReadInput};

    #[test]
    fn gives_back_trimmed_string() -> std::io::Result<()> {
        let bytes = b"  I have spaces in this string  ";
        let mut reader = Reader::new(&bytes[..]);

        let actual = reader.read_input().unwrap();
        let expected = "I have spaces in this string".to_string();
        assert_eq!(actual, expected);

        Ok(())
    }
}
