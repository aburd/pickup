extern crate dirs;

use crate::printer::Print;
use crate::reader::ReadInput;
use crate::storage::{FileStorage, Store};
use log::{info, trace, warn};
use std::io;
use std::fmt::Display;

pub mod printer;
pub mod reader;
pub mod storage;

pub struct Pickup<R, P, S>
where
    R: ReadInput,
    P: Print,
    S: Store,
{
    reader: R,
    printer: P,
    pub storage: S,
}

impl<R: ReadInput, P: Print, S: Store> Pickup<R, P, S> {
    pub fn new(reader: R, printer: P, storage: S) -> Self {
        Pickup { reader, printer, storage }
    }

    pub fn run(&mut self, opts: PickupOpts) -> io::Result<()> {
        trace!("Running pickup...");

        trace!("Loading items from config...");
        self.storage.load_items();
        let items = self.storage.get_items()?;
        
        for item in items {
            println!("{}", item);
        }

        info!("Exiting.");
        Ok(())
    }
}

pub struct PickupOpts {}
