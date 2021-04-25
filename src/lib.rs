use crate::printer::Print;
use crate::reader::ReadInput;

pub mod printer;
pub mod reader;

pub struct Pickup<R, P>
where
    R: ReadInput,
    P: Print,
{
    reader: R,
    printer: P,
}
