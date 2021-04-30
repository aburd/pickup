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

enum Command {
    ShowItems,
}

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
        self.storage.load_items()?;

        loop {
            self.print_options()?;
            self.printer.print("> ")?;
            if let Ok(cmd_d) = self.reader.read_input() {
                self.printer.println("")?;
                match cmd_d.as_str() {
                    "show" => self.show_items()?,
                    "exit" => {
                        break;
                    }
                    _ => (),
                }
            }
        }

        info!("Exiting.");
        Ok(())
    }

    fn show_items(&self) -> io::Result<()> {
        let items = self.storage.get_items()?;
        for item in items {
            println!("{}", item);
        }

        Ok(())
    }

    fn print_options(&mut self) -> io::Result<()> {
        self.printer.println("Select an option:")?;
        self.printer.println("show: Show all my items to pickup")?;
        self.printer.println("exit: Exit the program")?;

        Ok(())
    }
}

pub struct PickupOpts {}
