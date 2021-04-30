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
    AddItem(String),
    ShowItem(usize),
    RemoveItem(usize),
    Exit,
}

#[derive(Debug)]
pub struct PickupOpts {
    pub list_items: bool,
    pub add_item: (bool, String),
    pub show_item: (bool, usize),
    pub remove_item: (bool, usize),
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
        let cmds_from_opts = self.process_opts(opts);

        if cmds_from_opts.len() > 0 {
            for cmd in cmds_from_opts {
                self.process_command(cmd)?;
            }
        } else {
            let _res = self.run_with_no_opts();
        }

        info!("Exiting.");
        Ok(())
    }

    fn process_opts(&mut self, opts: PickupOpts) -> Vec<PickupCommand> {
        debug!("Processing opts: {:?}", opts);
        let mut cmds = vec![];
        if opts.add_item.0 {
            cmds.push(PickupCommand::AddItem(opts.add_item.1));
        }
        if opts.list_items {
            cmds.push(PickupCommand::ListItems);
        }
        if opts.show_item.0 {
            cmds.push(PickupCommand::ShowItem(opts.show_item.1));
        }
        if opts.remove_item.0 {
            cmds.push(PickupCommand::RemoveItem(opts.remove_item.1));
        }
        cmds
    }

    fn get_command(&mut self) -> io::Result<PickupCommand> {
        let user_input = self.reader.read_input()?;
        match user_input.as_str() {
            "add" => {
                let name = self.reader.read_input()?;
                Ok(PickupCommand::AddItem(name))
            }
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

    fn print_options(&mut self) -> io::Result<()> {
        self.printer.println("add: Add item")?;
        self.printer.println("list: List items")?;
        self.printer.println("item: Show item")?;
        self.printer.println("remove: Remove item")?;
        self.printer.println("exit: exit pickup")?;

        Ok(())
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

        loop {
            self.print_options()?;
            self.printer.print("> ")?;
            if let Ok(command) = self.get_command() {
                debug!("Got command: {:?}", command);

                let processed = self.process_command(command)?;
                if !processed {
                    break;
                }

                self.printer.println("")?;
            } else {
                self.printer.println("That was an invalid command.")?;
                self.print_options()?;
            }
        }

        Ok(())
    }

    fn process_command(&mut self, command: PickupCommand) -> io::Result<bool> {
        debug!("Processing command: {:?}", command);
        match command {
            PickupCommand::AddItem(name) => {
                let item = self.storage.add_item(name)?;
                self.printer.println("Added item")?;
                self.printer.println(&item.to_string())?;
                Ok(true)
            }
            PickupCommand::ListItems => {
                self.list_items()?;
                Ok(true)
            }
            PickupCommand::ShowItem(id) => {
                let item = self.storage.get_item(id as u64)?;
                self.printer.println(&item.to_string())?;
                Ok(true)
            }
            PickupCommand::RemoveItem(id) => {
                let id = self.storage.remove_item(id as u64)?;
                self.printer
                    .println(&format!("Item with id {} removed.", id))?;
                Ok(true)
            }
            PickupCommand::Exit => Ok(false),
        }
    }
}
