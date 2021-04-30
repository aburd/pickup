extern crate dirs;

use crate::printer::Print;
use crate::reader::ReadInput;
use crate::storage::Store;
use log::{debug, info, trace};
use std::io;

pub mod printer;
pub mod reader;
pub mod storage;

#[derive(Debug)]
pub enum PickupCommand {
    ListItems,
    ShowItem(usize),
    RemoveItem(usize),
    Exit,
}

#[derive(Debug)]
pub struct PickupOpts {
    pub list_items: bool,
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

        let mut opts_passed = false;
        debug!("Checking opts...");
        debug!("Options {:?}", opts);
        if opts.list_items {
            opts_passed = true;
            self.list_items()?;
        }

        if !opts_passed {
            self.run_with_no_opts()?;
        }

        info!("Exiting.");
        Ok(())
    }

    fn get_command(&mut self) -> io::Result<PickupCommand> {
        let user_input = self.reader.read_input()?;
        match user_input.as_str() {
            "list" | "ls" | "items" => Ok(PickupCommand::ListItems),
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

    fn list_items(&mut self) -> io::Result<()> {
        let items = self.storage.get_items()?;
        for item in items {
            self.printer.println(&item.to_string())?;
        }
        Ok(())
    }

    fn run_with_no_opts(&mut self) -> io::Result<()> {
        self.printer.println("What would you like to do?")?;
        self.printer.print("> ")?;

        loop {
            let command = self.get_command()?;
            debug!("Got command: {:?}", command);

            match command {
                PickupCommand::ListItems => {
                    self.list_items()?;
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
            self.printer.println("")?;
        }

        Ok(())
    }
}
