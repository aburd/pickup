extern crate dirs;

use crate::printer::Print;
use crate::reader::ReadInput;
use log::{info, trace, warn};
use std::io;

pub mod printer;
pub mod reader;
pub mod storage;
pub mod user_config;

pub struct Pickup<R, P>
where
    R: ReadInput,
    P: Print,
{
    reader: R,
    printer: P,
}

impl<R: ReadInput, P: Print> Pickup<R, P> {
    pub fn new(reader: R, printer: P) -> Self {
        Pickup { reader, printer }
    }

    pub fn run(&mut self, opts: PickupOpts) -> io::Result<()> {
        trace!("Running pickup...");

        self.printer.println("I can print stuff!")?;

        info!("Exiting.");
        Ok(())
    }
}

pub struct PickupOpts {}
