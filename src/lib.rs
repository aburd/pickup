use std::io;
use log::{trace, info, warn};
use crate::printer::{Print};
use crate::reader::{ReadInput};

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

impl <R: ReadInput, P: Print> Pickup<R, P> {
    pub fn new(reader: R, printer: P) -> Self {
        Pickup {
            reader,
            printer,
        }
    }


pub fn run(&mut self, opts: PickupOpts) -> io::Result<()> {
    trace!("Running pickup...");

    self.printer.println("I can print stuff!")?;

    info!("Exiting.");
    Ok(())
} 
}

pub struct PickupOpts {}
