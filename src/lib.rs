extern crate dirs;

use crate::printer::Print;
use crate::reader::ReadInput;
use crate::storage::{FileStorage, Store};
use log::{debug, info, trace};
use std::io;

pub mod printer;
pub mod reader;
pub mod storage;

#[derive(Debug)]
pub enum PickupCommand {
    ShowItems,
    ShowItem(usize),
    RemoveItem(usize),
    Exit,
}

#[derive(Debug)]
pub struct PickupOpts {}

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
        Pickup {
            reader,
            printer,
            storage,
        }
    }

    pub fn run(&mut self, opts: PickupOpts) -> io::Result<()> {
        trace!("Running pickup...");

        trace!("Loading items from config...");
        self.storage.load_items()?;
        self.printer.println("What would you like to do?")?;
        self.printer.print("> ")?;

        loop {
            let command = self.get_command()?;
            debug!("Got command: {:?}", command);

            match command {
                PickupCommand::ShowItems => {
                    let items = self.storage.get_items()?;
                    for item in items {
                        self.printer.println(&item.to_string())?;
                    }
                }
                PickupCommand::ShowItem(id) => {
                    let item = self.storage.get_item(id as u64)?;
                    self.printer.println(&item.to_string())?;
                }
                PickupCommand::RemoveItem(id) => {
                    self.storage.remove_item(id as u64)?;
                    self.printer.println("Item removed.")?;
                }
                PickupCommand::Exit => {
                    break;
                }
            }
        }

        self.printer.println("")?;

        info!("Exiting.");
        Ok(())
    }

    fn get_command(&mut self) -> io::Result<PickupCommand> {
        let user_input = self.reader.read_input()?;
        match user_input.as_str() {
            "items" => Ok(PickupCommand::ShowItems),
            "item" => {
                let id = self.get_id()?;
                Ok(PickupCommand::ShowItem(id))
            }
            "remove" => {
                let id = self.get_id()?;
                Ok(PickupCommand::RemoveItem(id))
            }
            "exit" => Ok(PickupCommand::Exit),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input")),
        }
    }

    fn get_id(&mut self) -> io::Result<usize> {
        self.printer.println("Which ID?")?;
        self.printer.print(">")?;

        let id_input = self.reader.read_input()?;
        let id = id_input.parse::<usize>().unwrap();
        Ok(id)
    }
}
