
use crate::reader::ReadInput;
use crate::printer::Print;

pub mod reader;
pub mod printer;

pub struct Pickup<R, P> where R: ReadInput, P: Print {
    reader: R,
    printer: P,
}